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

use crate::{
    extension::{
        Extension, ExtensionFactories, ExtensionMetaInfo,
        LoadExtensionPromise,
    },
    params::extension_param::{ExtensionName, ExtensionType},
    url::UrlParam,
    StdError, Url,
};
use async_trait::async_trait;
use bytes::Bytes;
use futures_core::Stream;
use std::{collections::HashMap, future::Future, marker::PhantomData, pin::Pin};
use std::any::Any;
use thiserror::Error;


// url: invoker://127.0.0.1:8080?invoker-name=hello_service_invoker&invoker-protocol=trip&invoker-service-name=hello_service
// extension_url: extension://0.0.0.0?extension-type=invoker&extension-name=trip-invoker&extension-url=invoker://127.0.0.1:8080?invoker-name=hello_service_invoker&invoker-protocol=trip&invoker-service-name=hello_service
#[async_trait]
pub trait Invoker {
    async fn invoke(
        &mut self,
        invocation: GrpcInvocation,
    ) -> Result<Pin<Box<dyn Stream<Item = Bytes> + Send + 'static>>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> Result<Url, StdError>;

    fn clone(&self) -> Box<dyn Invoker + Send + Sync + 'static>;
}

impl Clone for Box<dyn Invoker + Send + Sync + 'static> {
    fn clone(&self) -> Box<dyn Invoker + Send + Sync + 'static> {
        Invoker::clone(self.as_ref())
    }
}

pub enum CallType {
    Unary,
    ClientStream,
    ServerStream,
    BiStream,
}

pub struct GrpcInvocation {
    service_name: String,
    method_name: String,
    arguments: Vec<Argument>,
    attachments: HashMap<String, String>,
    call_type: CallType,
}

pub struct Argument {
    name: String,
    value: Box<dyn Stream<Item = Box<dyn Serializable + Send + 'static>> + Send + 'static>,
}

pub trait Serializable {
    fn serialize(&self, serialization_type: String) -> Result<Bytes, StdError>;

    fn into_any(self) -> Box<dyn Any + Send + 'static>;
}

pub trait Deserializable {
    fn deserialize(&self, bytes: Bytes, deserialization_type: String) -> Result<Self, StdError>
    where
        Self: Sized;
}


#[derive(Default)]
pub(super) struct InvokerExtensionLoader {
    factories: HashMap<String, InvokerExtensionFactory>,
}

impl InvokerExtensionLoader {
    pub fn register(&mut self, extension_name: String, factory: InvokerExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }

    pub fn remove(&mut self, extension_name: String) {
        self.factories.remove(&extension_name);
    }

    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Invoker + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();
        let Some(extension_name) = extension_name else {
            return Err(InvokerExtensionLoaderError::new(
                "load invoker extension failed, extension mustn't be empty",
            )
            .into());
        };
        let extension_name = extension_name.value();
        let factory = self.factories.get_mut(&extension_name);
        let Some(factory) = factory else {
            let err_msg = format!(
                "load {} invoker extension failed, can not found extension factory",
                extension_name
            );
            return Err(InvokerExtensionLoaderError(err_msg).into());
        };
        factory.create(url)
    }
}

type InvokerExtensionConstructor = fn(
    Url,
) -> Pin<
    Box<dyn Future<Output = Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>> + Send + 'static>,
>;
pub(crate) struct InvokerExtensionFactory {
    constructor: InvokerExtensionConstructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn Invoker + Send + Sync + 'static>>>,
}

impl InvokerExtensionFactory {
    pub fn new(constructor: InvokerExtensionConstructor) -> Self {
        Self {
            constructor,
            instances: HashMap::default(),
        }
    }
}

impl InvokerExtensionFactory {
    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Invoker + Send + Sync + 'static>>, StdError> {
        let key = url.to_string();

        match self.instances.get(&key) {
            Some(instance) => Ok(instance.clone()),
            None => {
                let constructor = self.constructor;
                let creator = move |url: Url| {
                    Box::pin(async move {
                        let invoker = constructor(url).await?;
                        Ok(invoker)
                    })
                        as Pin<
                            Box<
                                dyn Future<Output = Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>>
                                    + Send
                                    + 'static,
                            >,
                        >
                };

                let promise = LoadExtensionPromise::new(Box::new(creator), url);
                self.instances.insert(key, promise.clone());
                Ok(promise)
            }
        }
    }
}

pub struct InvokerExtension<T>(PhantomData<T>)
where
    T: Invoker + Send + 'static;

impl<T> ExtensionMetaInfo for InvokerExtension<T>
where
    T: Invoker + Send + 'static,
    T: Extension<Target = Box<dyn Invoker + Send + Sync + 'static>>,
{
    fn name() -> String {
        T::name()
    }

    fn extension_type() -> ExtensionType {
        ExtensionType::Invoker
    }

    fn extension_factory() -> ExtensionFactories {
        ExtensionFactories::InvokerExtensionFactory(InvokerExtensionFactory::new(
            <T as Extension>::create,
        ))
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct InvokerExtensionLoaderError(String);

impl InvokerExtensionLoaderError {
    pub fn new(msg: &str) -> Self {
        InvokerExtensionLoaderError(msg.to_string())
    }
}
