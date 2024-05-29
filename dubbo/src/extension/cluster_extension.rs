use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;
use thiserror::Error;

use crate::common::url::params::extension_params::{ExtensionName, ExtensionUrl};
use crate::common::url::{Url, UrlParam};
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::StdError;

use super::protocol_extension::Invoker;
use super::LoadExtensionPromise;

// url: cluster://0.0.0.0?cluster-name=failover-cluster&cluster-service-name=hello&cluster-type=failover
// extension_url: extension://0.0.0.1?extension-type=cluster&extension-name=failover-cluster&extension-url=cluster://0.0.0.0?cluster-name=failover-cluster&cluster-service-name=hello&cluster-type=failover
#[async_trait]
pub trait Cluster{
    
    async fn join(&mut self, load_balancer: Box<dyn LoadBalancerChooser + Send + Sync + 'static>) -> Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn Cluster + Send + Sync + 'static>;
}

impl Clone for Box<dyn Cluster + Send + Sync + 'static> {
    fn clone(&self) -> Box<dyn Cluster + Send + Sync + 'static> {
        Cluster::clone(self.as_ref())
    }
}


pub fn build_cluster_url() -> Url {
    let mut url = Url::empty();
    url.set_protocol("cluster");
    url.set_host("0.0.0.0");
    url
}



#[derive(Default)]
pub struct ClusterExtensionLoader {
    factories: HashMap<String, ClusterExtensionFactory>,
}

impl ClusterExtensionLoader {

    pub fn new() -> Self {
        Self {
            factories: HashMap::default(),
        }
    }


    pub fn register(&mut self, extension_name: String, factory: ClusterExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }


    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }

    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Cluster + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();

        let Some(extension_name) = extension_name else {
            return Err(ClusterExtensionLoaderError::new("load cluster extension failed, cluster extension type mustn't be empty").into());
        };

        let extension_name = extension_name.as_str();

        let factory = self.factories.get_mut(extension_name.as_ref());
        let Some(factory) = factory else {
            let err_msg = format!(
                "load {} cluster extension failed, can not found cluster extension factory",
                extension_name
            );
            return Err(ClusterExtensionLoaderError(err_msg).into());
        };
        factory.create(url)
    }

}



type ClusterExtensionConstructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Cluster + Send + Sync + 'static>, StdError>> + Send + 'static>>;

pub struct ClusterExtensionFactory {

    constructor: ClusterExtensionConstructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn Cluster + Send + Sync + 'static>>>,
}


impl ClusterExtensionFactory {


    pub fn new(constructor: ClusterExtensionConstructor) -> Self {
        Self {
            constructor,
            instances: HashMap::default(),
        }
    }


    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Cluster + Send + Sync + 'static>>, StdError> {
        let extension_url = url.query::<ExtensionUrl>();

        let Some(extension_url) = extension_url else {
            return Err(ClusterExtensionLoaderError::new("load cluster extension failed, clusten extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();

        match self.instances.get(extension_url_str.as_ref()) {
            Some(instance) => Ok(instance.clone()),
            None => {
                let constructor = self.constructor;
                let creator = move |url: Url| {
                    Box::pin(async move {
                        let cluster = constructor(url).await?;
                        Ok(cluster)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn Cluster + Send + Sync + 'static>, StdError>> + Send + 'static>>
                };
                
                let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
                self.instances.insert(extension_url_str.into_owned(), promise.clone());
                Ok(promise)
            }
        }
    }

}


#[derive(Error, Debug)]
#[error("{0}")]
pub struct ClusterExtensionLoaderError(String);


impl ClusterExtensionLoaderError {

    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}