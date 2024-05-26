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
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_core::ready;
use thiserror::Error;
use tower::MakeService;
use tower_layer::Layer;
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::invoker_directory_extension::InvokerList;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::extension::route_extension::Router;
use crate::params::extension_params::{ExtensionName, ExtensionType, ExtensionUrl};
use crate::params::invoke_params::InvokeServiceName;
use crate::params::load_balancer_params::{LoadBalancerName, LoadBalancerServiceName, LoadBalancerType};
use crate::url::UrlParam;
use crate::{extension, StdError, Url};

pub struct MkLoadBalancerBuilder<N, M> {
    mk_invoker_list_builder: N,
    mk_router_builder: M,
}



impl<N, M> MkLoadBalancerBuilder<N, M> 
where
    M: Clone
{

    pub fn layer(mk_router_builder: M) -> impl Layer<N, Service = MkLoadBalancerBuilder<N, M>> {
       tower_layer::layer_fn(move |mk_invoker_list_builder: N| {
            MkLoadBalancerBuilder {
                mk_invoker_list_builder,
                mk_router_builder: mk_router_builder.clone()
            }
        })
    }
}


impl<N, M> Service<DubboConfig> for MkLoadBalancerBuilder<N, M> 
where
    N: MakeService<DubboConfig, Url, MakeError = StdError>,
    N::Future: Send + 'static,
    N::Service: Service<Url, Response = Box<dyn InvokerList + Send + Sync + 'static>, Error = StdError> + Send + 'static,
    <N::Service as Service<Url>>::Future: Send + 'static,
    M: MakeService<DubboConfig, Url, MakeError = StdError>,
    M::Future:  Send + 'static,
    M::Service: Service<Url, Response = Box<dyn Router + Send + Sync + 'static>, Error = StdError> + Send + 'static,
    <M::Service as Service<Url>>::Future: Send + 'static,
{
    
    type Response = LoadBalancerBuilder<N::Service, M::Service>;

    type Error = StdError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let _ = ready!(self.mk_invoker_list_builder.poll_ready(cx));
        self.mk_router_builder.poll_ready(cx)
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let load_balancer_type = req.load_balancer();
        let invoker_list_builder_fut = self.mk_invoker_list_builder.make_service(req.clone());
        let router_builder_fut = self.mk_router_builder.make_service(req.clone());
        
        
        let fut = async move {
            let invoker_list_builder = invoker_list_builder_fut.await?;
            let router_builder = router_builder_fut.await?;
            let load_balancer_builder = LoadBalancerBuilder::new(invoker_list_builder, router_builder, load_balancer_type);
            Ok(load_balancer_builder)
        };

        Box::pin(fut)

    }
}


pub struct LoadBalancerBuilder<N, M> {
    invoker_list_builder: N,
    router_builder: M,
    load_balancer_type: LoadBalancerType
}


impl<N, M> LoadBalancerBuilder<N, M> {

    pub fn new(invoker_list_builder: N, router_builder: M, load_balancer_type: LoadBalancerType) -> Self {
        LoadBalancerBuilder {
            invoker_list_builder,
            router_builder,
            load_balancer_type
        }
    }
}


impl<N, M> Service<Url> for LoadBalancerBuilder<N, M> 
where
    N: Service<Url, Response = Box<dyn InvokerList + Send + Sync + 'static>, Error = StdError>,
    N::Future: Future<Output = Result<N::Response, N::Error>> + Send + 'static,
    M: Service<Url, Response = Box<dyn Router + Send + Sync + 'static>, Error = StdError>,
    M::Future: Future<Output = Result<M::Response, M::Error>> + Send + 'static,
{
    
    type Response = Box<dyn LoadBalancerChooser + Send + Sync + 'static>;

    type Error = StdError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let _ = ready!(self.invoker_list_builder.poll_ready(cx));
        self.router_builder.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        
        let invoke_service_name = req.query::<InvokeServiceName>();
        let Some(invoke_service_name) = invoke_service_name else {
            return Box::pin(async move {
                return Err(LoadBalancerBuildError::new("service name mustn't be empty").into());
            });
        };

        let invoke_service_name = invoke_service_name.value();


        // build load balancer url
        let load_balancer_name = LoadBalancerName::new(format!("{}-{}", invoke_service_name, self.load_balancer_type.as_str()));
        let load_balancer_service_name = LoadBalancerServiceName::new(invoke_service_name.as_str());
        let load_balancer_type = self.load_balancer_type.clone();
        let mut load_balancer_url = extension::loadbalancer_extension::build_load_balancer_url(invoke_service_name.as_str());
        load_balancer_url.add_query_param(load_balancer_name);
        load_balancer_url.add_query_param(load_balancer_service_name);
        load_balancer_url.add_query_param(load_balancer_type);

        // build extension url
        let extension_name = ExtensionName::new(self.load_balancer_type.as_str());
        let extension_url = ExtensionUrl::new(load_balancer_url);
        let mut load_balancer_extension_url = extension::build_extension_url(ExtensionType::LoadBalancer, extension_name);
        load_balancer_extension_url.add_query_param(extension_url);

        // build load balancer
        let invoker_list_fut = self.invoker_list_builder.call(req.clone());
        let router_fut = self.router_builder.call(req.clone());

    

        let fut = async move {
            let invoker_list = invoker_list_fut.await?;
            let router = router_fut.await?;

            let mut load_balancer = extension::EXTENSIONS.load_load_balancer(load_balancer_extension_url).await?;
            let _ = load_balancer.ready().await?;
            let chooser = load_balancer.load_balancer(invoker_list, router).await?;
            Ok(chooser)
        };

        Box::pin(fut)
    }
}


impl<N: Clone, M: Clone> Clone for LoadBalancerBuilder<N, M> {
    fn clone(&self) -> Self {
        Self { invoker_list_builder: self.invoker_list_builder.clone(), router_builder: self.router_builder.clone(), load_balancer_type: self.load_balancer_type.clone() }
    }
}


#[derive(Debug, Error)]
#[error("build loadbalancer occur an error: {0}")]
pub struct LoadBalancerBuildError(String);


impl LoadBalancerBuildError {
    pub fn new(msg: impl Into<String>) -> Self {
        LoadBalancerBuildError(msg.into())
    }
}