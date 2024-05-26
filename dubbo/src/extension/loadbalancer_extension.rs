use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;
use thiserror::Error;
use crate::extension::invoker_directory_extension::InvokerList;
use crate::extension::route_extension::Router;
use crate::params::extension_params::{ExtensionName, ExtensionUrl};
use crate::url::UrlParam;
use crate::{StdError, Url};

use super::protocol_extension::Invoker;
use super::LoadExtensionPromise;

// url: load-balancer://service_name?load-balancer-name=random&load-balancer-service-name=hello&load-balancer-type=random
// extension_url: extension://0.0.0.0?extension-type=load-balancer&extension-name=random&extension-url=load-balancer://service_name?load-balancer-name=random&load-balancer-service-name=hello&load-balancer-type=random
#[async_trait]
pub trait LoadBalancer {

    async fn load_balancer(&mut self, invoker_list: Box<dyn InvokerList + Send + Sync + 'static>, router: Box<dyn Router + Send + Sync + 'static>) -> Result<Box<dyn LoadBalancerChooser + Send + Sync + 'static>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn LoadBalancer + Send + Sync + 'static>;
}



// url: load-balancer-chooser://service_name?load-balancer-chooser-name=random-chooser&load-balancer-chooser-service-name=hello&load-balancer-chooser-type=random-load-balancer-chooser
#[async_trait]
pub trait LoadBalancerChooser {

    async fn choose(&mut self) -> Box<dyn Invoker + Send + Sync + 'static>;

    async fn ready(&mut self) -> Result<(), StdError>;
    
    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn LoadBalancerChooser + Send + Sync + 'static>;
}


impl Clone for Box<dyn LoadBalancer + Send + Sync + 'static> {
    
    fn clone(&self) -> Self {
        LoadBalancer::clone(self.as_ref())
    }
}


pub fn build_load_balancer_url(service_name: &str) -> Url {
    let mut url = Url::empty();
    url.set_protocol("load-balancer");
    url.set_host(service_name);
    url
}


#[derive(Default)]
pub struct LoadBalancerExtensionLoader {
    factories: HashMap<String, LoadBalancerExtensionFactory>,
}


impl LoadBalancerExtensionLoader {

    pub fn register(&mut self, extesnion_name: String, factory: LoadBalancerExtensionFactory) {
        self.factories.insert(extesnion_name, factory);
    }


    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }


    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn LoadBalancer + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();
        let Some(extension_name) = extension_name else {
            return Err(LoadBalancerExtensionLoaderError::new("load loadBalancer extension failed, loadBalancer extension name mustn't be empty").into());
        };
        let extension_name = extension_name.as_str();

        let factory = self.factories.get_mut(extension_name.as_ref());
        
        let Some(factory) = factory else {
            return Err(LoadBalancerExtensionLoaderError::new(format!("load {} loadBalancer extension failed, can not found loadBalancer extension factory", extension_name)).into());
        };
        factory.create(url)
    }
}


type Constructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn LoadBalancer + Send + Sync + 'static>, StdError>> + Send + 'static>>;
pub struct LoadBalancerExtensionFactory {
    constructor: Constructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn LoadBalancer + Send + Sync + 'static>>>
}


impl LoadBalancerExtensionFactory {

    pub fn new(constructor: Constructor) -> Self {
        Self {
            constructor,
            instances: HashMap::default(),
        }
    }

    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn LoadBalancer + Send + Sync + 'static>>, StdError> {
        let extension_url = url.query::<ExtensionUrl>();
        let Some(extension_url) = extension_url else {
            return Err(LoadBalancerExtensionLoaderError::new("load loadBalancer extension failed, loadBalancer extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();
        let instance = self.instances.get(extension_url_str.as_ref());
        match instance {
            Some(instance) => {
                return Ok(instance.clone());
            }
            None => {
                let constructor = self.constructor;

                let creator = move |url: Url| {
                    Box::pin(async move {
                        let load_balancer = constructor(url).await?;
                        Ok(load_balancer)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn LoadBalancer + Send + Sync + 'static>, StdError>> + Send + 'static>>
                };

                let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
                self.instances.insert(extension_url_str.to_string(), promise.clone());
                Ok(promise)
            }
        }
    }
}


#[derive(Debug, Error)]
#[error("load balancer extension error: {0}")]
pub struct LoadBalancerExtensionLoaderError(String);

impl LoadBalancerExtensionLoaderError {
    pub fn new(msg: impl Into<String>) -> Self {
        LoadBalancerExtensionLoaderError(msg.into())
    }
}