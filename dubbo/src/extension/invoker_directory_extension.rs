use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use thiserror::Error;
use crate::extension::registry_extension::Registry;
use crate::params::extension_params::{ExtensionName, ExtensionUrl};
use crate::url::UrlParam;
use crate::{StdError, Url};

use super::protocol_extension::Invoker;
use super::LoadExtensionPromise;


// url: invoker-directory://service_name?invoker-directory-name=hello-default-invoker-directory&invoker-directory-service-name=hello&invoker-directory-type=invoker-directory
// extension_url: extension://0.0.0.0?extension-type=invoker-directory&extension-name=invoker-directory&extension-url=invoker-directory://service_name?invoker-directory-name=hello-default-invoker-directory&invoker-directory-service-name=hello&invoker-directory-type=invoker-directory
type Converter = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>>>>;
#[async_trait]
pub trait InvokerDirectory {
    
    async fn directory(&mut self, converter: Converter, registry: Box<dyn Registry + Send + Sync + 'static>) -> Result<Box<dyn InvokerList + Send + Sync + 'static>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn InvokerDirectory + Send + Sync + 'static>;
    
}


// url: invoker-list://service_name?service-name=hello&type=invoker-list
#[async_trait]
pub trait InvokerList {
    
    async fn list(&mut self) -> Result<Vec<Arc<dyn Invoker + Send + Sync + 'static>>, StdError>;

    async fn ready(&mut self) -> Result<(), crate::StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn InvokerList + Send + Sync + 'static>;

}



impl Clone for Box<dyn InvokerDirectory + Send + Sync + 'static> {
            
    fn clone(&self) -> Self {
        InvokerDirectory::clone(self.as_ref())
    }
}


pub fn build_invoker_directory_url(service_name: &str) -> Url {
    let mut url = Url::empty();
    url.set_protocol("invoker-directory");
    url.set_host(service_name);
    url
}


#[derive(Default)]
pub struct InvokerDirectoryExtensionLoader {
    factories: HashMap<String, InvokerDirectoryExtensionFactory>,
}


impl InvokerDirectoryExtensionLoader {
    
    pub fn new() -> Self {
        InvokerDirectoryExtensionLoader {
            factories: HashMap::new(),
        }
    }
}


impl InvokerDirectoryExtensionLoader {

    pub fn register(&mut self, extension_name: String, factory: InvokerDirectoryExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }

    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }


    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn InvokerDirectory + Send + Sync + 'static>>, StdError> {

        let extension_name = url.query::<ExtensionName>();

        let Some(extension_name) = extension_name else {
            return Err(InvokerDirectoryExtensionLoaderError::new("load invokerFirectory extension failed, invokerFirectory extension name mustn't be empty").into());
        };

        let extension_name = extension_name.value();
        let factory = self.factories.get_mut(&extension_name);
        let Some(factory) = factory else {
            return Err(InvokerDirectoryExtensionLoaderError::new(format!("load {} invokerFirectory extension failed, can not found invokerFirectory extension factory", extension_name)).into());
        };

        factory.create(url)

    }
}



type Constructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn InvokerDirectory + Send + Sync + 'static>, StdError>> + Send + 'static>>;
pub struct InvokerDirectoryExtensionFactory {
    constructor: Constructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn InvokerDirectory + Send + Sync + 'static>>>,
}


impl InvokerDirectoryExtensionFactory {

    pub fn new(constructor: Constructor) -> Self {
        InvokerDirectoryExtensionFactory {
            constructor,
            instances: HashMap::new(),
        }
    }
}


impl InvokerDirectoryExtensionFactory {


    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn InvokerDirectory + Send + Sync + 'static>>, StdError> {
        let extension_url = url.query::<ExtensionUrl>();
        let Some(extension_url) = extension_url else {
            return Err(InvokerDirectoryExtensionLoaderError::new("load invokerFirectory extension failed, invokerFirectory extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();

        let instance = self.instances.get(extension_url_str.as_ref());
        match instance {
            Some(instance) => Ok(instance.clone()),
            None => {
               let constructor = self.constructor;
               let creator = move |url: Url| {
                    Box::pin(async move {
                        let instance = constructor(url).await?;
                        Ok(instance)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn InvokerDirectory + Send + Sync + 'static>, StdError>> + Send + 'static>>
               };

               let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
               self.instances.insert(extension_url_str.into(), promise.clone());
               Ok(promise)
            }
        }
    }

}


#[derive(Debug, Error)]
#[error("load invoker directory extension failed, {0}")]
pub struct InvokerDirectoryExtensionLoaderError(String);

impl InvokerDirectoryExtensionLoaderError {
    pub fn new(msg: impl Into<String>) -> Self {
        InvokerDirectoryExtensionLoaderError(msg.into())
    }
}