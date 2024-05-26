/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::pin::Pin;

use futures::Future;
use thiserror::Error;
use tower::{Layer, MakeService, Service};

use crate::{config::dubbo_config::DubboConfig, extension::{self, invoker_directory_extension::InvokerList, protocol_extension::Invoker, registry_extension::Registry}, params::{extension_params::{ExtensionName, ExtensionType, ExtensionUrl}, invoke_params::InvokeServiceName, invoker_direcotry_params::{InvokerDirectoryName, InvokerDirectoryServiceName, InvokerDirectoryType}, invoker_params::InvokerProtocol, protocol_params::ProtocolType}, url::UrlParam, StdError, Url};

pub struct MkInvokerDirectoryBuilder<N> {
    inner: N,
}


impl<N> MkInvokerDirectoryBuilder<N> {

    pub fn layer() -> impl Layer<N, Service = Self> {
        tower_layer::layer_fn(|inner| MkInvokerDirectoryBuilder { inner })
    }
}


impl<N> Service<DubboConfig> for MkInvokerDirectoryBuilder<N> 
where
    N: MakeService<DubboConfig, Url, MakeError = StdError>,
    N::Future: Send + 'static,
    N::Service: Service<Url, Response = Box<dyn Registry + Send + Sync + 'static>, Error = StdError> + Send + 'static,
    <N::Service as Service<Url>>::Future: Send + 'static,
{
    
    type Response = InvokerDirectoryBuilder<N::Service>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;
    
    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let invoker_directory_type = req.invoker_directory();
        let registry_builder = self.inner.make_service(req);
        let fut = async move {
            let registry_builder = registry_builder.await?;
            Ok(InvokerDirectoryBuilder {
                inner: registry_builder,
                invoker_directory_type,
            })
        };
        Box::pin(fut)
    }

}

pub struct InvokerDirectoryBuilder<N> {
    inner: N,
    invoker_directory_type: InvokerDirectoryType,
}


impl<N> Service<Url> for InvokerDirectoryBuilder<N> 
where
    N: Service<Url, Response = Box<dyn Registry + Send + Sync + 'static>, Error = StdError>,
    N::Future: Future<Output = Result<N::Response, N::Error>> + Send + 'static
{
    
    type Response = Box<dyn InvokerList + Send + Sync + 'static>;

    type Error = StdError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let invoke_service_name = req.query::<InvokeServiceName>();

        let Some(invoke_service_name) = invoke_service_name else {
            return Box::pin(async { Err(InvokerDirectoryBuilderError::new("service name is required").into()) });
        };

        let invoke_service_name = invoke_service_name.value();
        
         // build directory url
         let invoker_directory_name = InvokerDirectoryName::new(format!("{}-{}", invoke_service_name, self.invoker_directory_type.as_str()));
         let invoker_directory_service = InvokerDirectoryServiceName::new(&invoke_service_name);
         let mut invoker_directory_url = extension::invoker_directory_extension::build_invoker_directory_url(invoke_service_name.as_str());
         invoker_directory_url.add_query_param(invoker_directory_name);
         invoker_directory_url.add_query_param(invoker_directory_service);

         // build extension url
         let extension_name = ExtensionName::new(self.invoker_directory_type.as_str());
         let extension_url = ExtensionUrl::new(invoker_directory_url);
         let mut invoker_directory_extension_url = extension::build_extension_url(ExtensionType::InvokerDirectory, extension_name);
         invoker_directory_extension_url.add_query_param(extension_url);


         let registry_fut = self.inner.call(req);
        

        let fut = async move {

            let registry = registry_fut.await?;
           
            let mut load_invoker_directory = extension::EXTENSIONS.load_invoker_directory(invoker_directory_extension_url).await?;
            let _ = load_invoker_directory.ready().await?;


            let invoker_list = load_invoker_directory.directory(create_invoker, registry).await?;

            Ok(invoker_list)
        };
        Box::pin(fut)
    }
}


// invoker url: invoker://127.0.0.1:8080?invoker-name=hello-service-invoker&invoker-protocol=trip&invoker-service-name=hello_service
fn create_invoker(url: Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>>>> {
    Box::pin(async move {
        let invoker_protocol = url.query::<InvokerProtocol>();
        let Some(invoker_protocol) = invoker_protocol else {
            return Err(InvokerDirectoryBuilderError::new("the invoker protocol is empty").into());
        };
        let invoker_protocol = invoker_protocol.value();

        // build protocol url
        let protocol_type = ProtocolType::new(&invoker_protocol);
        let mut protocol_url = extension::protocol_extension::build_protocol_url();
        protocol_url.add_query_param(protocol_type);

        // build extension url
        let extension_name = ExtensionName::new(&invoker_protocol);
        let extension_url = ExtensionUrl::new(protocol_url);
        let mut protocol_extension_url = extension::build_extension_url(ExtensionType::Protocol, extension_name);
        protocol_extension_url.add_query_param(extension_url);

        // load protocol extension
        let mut protocol_extension = extension::EXTENSIONS.load_protocol(protocol_extension_url).await?;
        let _ = protocol_extension.ready().await?;

        let invoker = protocol_extension.reference(url).await?;
        Ok(invoker)
    })
}

impl<N: Clone> Clone for InvokerDirectoryBuilder<N> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), invoker_directory_type: self.invoker_directory_type.clone() }
    }
}

#[derive(Debug, Error)]
#[error("InvokerDirectoryBuilderError: {0}")]
pub struct InvokerDirectoryBuilderError(String);

impl InvokerDirectoryBuilderError {
    pub fn new(msg: impl Into<String>) -> Self {
        InvokerDirectoryBuilderError(msg.into())
    }
}