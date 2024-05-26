use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;
use thiserror::Error;
use crate::params::extension_params::{ExtensionName, ExtensionUrl};
use crate::url::UrlParam;
use crate::{StdError, Url};

use super::protocol_extension::Invoker;
use super::LoadExtensionPromise;

// url: route://service_name?reoute-name=route&reoute-service-name=hello&reoute-type=tag-router
// extension_url: extension://0.0.0.0?extension-type=route&extension-name=tag-router&extension-url=route://service_name?reoute-name=route&reoute-service-name=hello&reoute-type=tag-router
#[async_trait]
pub trait Router {
    async fn route(&mut self, invokers: Vec<Box<dyn Invoker + Send + Sync + 'static>>) -> Box<dyn Invoker + Send + Sync + 'static>;

    async fn ready(&mut self) -> Result<(), StdError>;
    
    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn Router + Send + Sync + 'static>;
}

impl Clone for Box<dyn Router + Send + Sync + 'static> {
        
    fn clone(&self) -> Self {
        Router::clone(self.as_ref())
    }
}


pub fn build_router_url(service_name: &str) -> Url {
    let mut url = Url::empty();
    url.set_protocol("route");
    url.set_host(service_name);
    url
}


#[derive(Default)]
pub struct RouterExtensionLoader {

    factories: HashMap<String, RouterExtensionFactory>,
}

impl RouterExtensionLoader {

    pub fn new() -> Self {
        RouterExtensionLoader {
            factories: HashMap::new(),
        }
    }
}


impl RouterExtensionLoader {

    pub fn register(&mut self, extension_name: String, factory: RouterExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }


    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }


    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Router + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();

        let Some(extension_name) = extension_name else {
            return Err(RouterExtensionLoaderError::new("load router extension failed, router extension name mustn't be empty").into());
        };

        let extension_name = extension_name.value();

        let factory = self.factories.get_mut(extension_name.as_str());

        let Some(factory) = factory else {
            return Err(RouterExtensionLoaderError::new(format!("load {} router extension failed, can not found router extension factory", extension_name)).into());
        };

        factory.create(url)
    }

}


type Constructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Router + Send + Sync + 'static>, StdError>> + Send + 'static>>;
pub struct RouterExtensionFactory {
    constructor: Constructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn Router + Send + Sync + 'static>>>,
}


impl RouterExtensionFactory {

    pub fn new(constructor: Constructor) -> Self {
        RouterExtensionFactory {
            constructor,
            instances: HashMap::new(),
        }
    }
}


impl RouterExtensionFactory {


    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Router + Send + Sync + 'static>>, StdError> {
        let extension_url = url.query::<ExtensionUrl>();
        let Some(extension_url) = extension_url else {
            return Err(RouterExtensionLoaderError::new("load router extension failed, router extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();

        let instance = self.instances.get(extension_url_str.as_ref());

        match instance {
            Some(instance) => {
                Ok(instance.clone())
            }
            None => {
                let constructor = self.constructor;
                let creator = move |url: Url| {
                    Box::pin(async move {
                        let router = (constructor)(url).await?;
                        Ok(router)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn Router + Send + Sync + 'static>, StdError>> + Send + 'static>>
                };

                let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
                self.instances.insert(extension_url_str.into(), promise.clone());
                Ok(promise)
            }
        }
    }

}



#[derive(Debug, Error)]
#[error("load router extension error: {0}")]
pub struct RouterExtensionLoaderError(String);

impl RouterExtensionLoaderError {

    pub fn new(msg: impl Into<String>) -> Self {
        RouterExtensionLoaderError(msg.into())
    }
}