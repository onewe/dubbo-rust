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

use std::{collections::HashMap, future::Future, marker::PhantomData, pin::Pin};

use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::mpsc::Receiver;
use tower::discover::Change;

use crate::{
    params::{extension_param::ExtensionName, registry_param::RegistryUrl},
    url::UrlParam,
    StdError, Url,
};

use crate::extension::{
    Extension, ExtensionFactories, ExtensionMetaInfo, ExtensionType, LoadExtensionPromise,
};

// extension://0.0.0.0/?extension-type=registry&extension-name=nacos&registry-url=nacos://127.0.0.1:8848
pub fn to_extension_url(registry_url: Url) -> Url {
    let mut registry_extension_loader_url: Url = "extension://0.0.0.0".parse().unwrap();

    let protocol = registry_url.protocol();

    registry_extension_loader_url.add_query_param(ExtensionType::Registry);
    registry_extension_loader_url.add_query_param(ExtensionName::new(protocol.to_string()));
    registry_extension_loader_url.add_query_param(RegistryUrl::new(registry_url));

    registry_extension_loader_url
}

pub type ServiceChange = Change<String, ()>;
pub type DiscoverStream = Receiver<Result<ServiceChange, StdError>>;

// url: registry://127.0.0.1:8848?protocol=nacos
// extension_url: extension://0.0.0.0?extension-type=registry&extension-name=nacos&registry-url=registry://127.0.0.1:8848?protocol=nacos
#[async_trait]
pub trait Registry {
    async fn register(&mut self, url: Url) -> Result<(), StdError>;

    async fn unregister(&mut self, url: Url) -> Result<(), StdError>;

    async fn subscribe(&mut self, url: Url) -> Result<DiscoverStream, StdError>;

    async fn unsubscribe(&mut self, url: Url) -> Result<(), StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn Registry + Send + 'static>;
}

pub struct RegistryExtension<T>(PhantomData<T>)
where
    T: Registry + Send + 'static;

impl<T> ExtensionMetaInfo for RegistryExtension<T>
where
    T: Registry + Send + 'static,
    T: Extension<Target = Box<dyn Registry + Send + 'static>>,
{
    fn name() -> String {
        T::name()
    }

    fn extension_type() -> ExtensionType {
        ExtensionType::Registry
    }

    fn extension_factory() -> ExtensionFactories {
        ExtensionFactories::RegistryExtensionFactory(RegistryExtensionFactory::new(
            <T as Extension>::create,
        ))
    }
}

#[derive(Default)]
pub(super) struct RegistryExtensionLoader {
    factories: HashMap<String, RegistryExtensionFactory>,
}

impl RegistryExtensionLoader {
    pub(crate) fn register(&mut self, extension_name: String, factory: RegistryExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }

    pub(crate) fn remove(&mut self, extension_name: String) {
        self.factories.remove(&extension_name);
    }

    pub(crate) fn load(
        &mut self,
        url: Url,
    ) -> Result<LoadExtensionPromise<Box<dyn Registry + Send + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>().unwrap();
        let extension_name = extension_name.value();
        let factory = self.factories.get_mut(&extension_name).ok_or_else(|| {
            RegistryExtensionLoaderError::new(format!(
                "registry extension loader error: extension name {} not found",
                extension_name
            ))
        })?;
        factory.create(url)
    }
}

type RegistryConstructor = fn(
    Url,
) -> Pin<
    Box<dyn Future<Output = Result<Box<dyn Registry + Send + Sync + 'static>, StdError>> + Send>,
>;

pub(crate) struct RegistryExtensionFactory {
    constructor: RegistryConstructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn Registry + Send + 'static>>>,
}

impl RegistryExtensionFactory {
    pub(super) fn new(constructor: RegistryConstructor) -> Self {
        Self {
            constructor,
            instances: HashMap::new(),
        }
    }
}

impl RegistryExtensionFactory {
    pub(super) fn create(
        &mut self,
        url: Url,
    ) -> Result<LoadExtensionPromise<Box<dyn Registry + Send + 'static>>, StdError> {
        let registry_url = url.query::<RegistryUrl>().unwrap();
        let registry_url = registry_url.value();
        let url_str = registry_url.as_str().to_string();
        match self.instances.get(&url_str) {
            Some(instance) => {
                let instance = instance.clone();
                Ok(instance)
            }
            None => {
                let constructor = self.constructor;

                let creator = move |url: Url| {
                    Box::pin(async move {
                        let registry = constructor(url).await?;
                        Ok(registry)
                    })
                        as Pin<
                            Box<
                                dyn Future<Output = Result<Box<dyn Registry + Send + 'static>, StdError>>
                                    + Send
                                    + 'static,
                            >,
                        >
                };

                let promise = LoadExtensionPromise::new(Box::new(creator), url);
                self.instances.insert(url_str, promise.clone());
                Ok(promise)
            }
        }
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub(crate) struct RegistryExtensionLoaderError(String);

impl RegistryExtensionLoaderError {
    pub(crate) fn new(msg: String) -> Self {
        RegistryExtensionLoaderError(msg)
    }
}
