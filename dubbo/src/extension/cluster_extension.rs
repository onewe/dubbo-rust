use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;
use thiserror::Error;

use crate::extension::invoker_extension::Invoker;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::params::cluster_params::ClusterName;
use crate::params::extension_param::ExtensionType;
use crate::url::UrlParam;
use crate::{StdError, Url};

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


    pub fn register(&mut self, cluster_extension_type: String, factory: ClusterExtensionFactory) {
        self.factories.insert(cluster_extension_type, factory);
    }


    pub fn remove(&mut self, cluster_extension_type: &str) {
        self.factories.remove(cluster_extension_type);
    }

    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Cluster + Send + Sync + 'static>>, StdError> {
        let cluster_extension_type = url.query::<ExtensionType>();

        let Some(cluster_extension_type) = cluster_extension_type else {
            return Err(ClusterExtensionLoaderError::new("load cluster extension failed, cluster extension type mustn't be empty").into());
        };

        let cluster_extension_type = cluster_extension_type.value();

        let factory = self.factories.get_mut(&cluster_extension_type);
        let Some(factory) = factory else {
            let err_msg = format!(
                "load {} cluster extension failed, can not found cluster extension factory",
                cluster_extension_type
            );
            return Err(ClusterExtensionLoaderError(err_msg).into());
        };
        factory.create(url)
    }

}



type ClusterExtensionConstructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Cluster + Send + Sync + 'static>, StdError>> + Send + 'static>>;

pub(super) struct ClusterExtensionFactory {

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
        let cluster_name = url.query::<ClusterName>();

        let Some(cluster_name) = cluster_name else {
            return Err(ClusterExtensionLoaderError::new("load cluster extension failed, cluster name mustn't be empty").into());
        };

        let cluster_name = cluster_name.value();

        match self.instances.get(&cluster_name) {
            Some(instance) => Ok(instance.clone()),
            None => {
                let constructor = self.constructor;
                let creator = move |url: Url| {
                    Box::pin(async move {
                        let cluster = constructor(url).await?;
                        Ok(cluster)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn Cluster + Send + Sync + 'static>, StdError>> + Send + 'static>>
                };
                
                let promise = LoadExtensionPromise::new(Box::new(creator), url);
                self.instances.insert(cluster_name, promise.clone());
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