use async_trait::async_trait;
use dubbo_base::Url;
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::{extension::{cluster_extension::Cluster, ClusterExtensionLoader}, invoker::{Invoker, RpcInvocation, RpcResponse}, StdError};

pub struct DefaultClusterLoader;

impl DefaultClusterLoader {

    pub const NAME: &'static str = "default-cluster";
}

#[async_trait::async_trait]
impl ClusterExtensionLoader for DefaultClusterLoader {

    fn name(&self) -> String {
        Self::NAME.to_string()
    }

    async fn load(&mut self, url: &Url) -> Result<Box<dyn Cluster + Send>, StdError> {

        let mut default_cluster_url = url.clone();
        default_cluster_url.set_protocol(DefaultClusterLoader::NAME);
        default_cluster_url.remove_all_param();

        let cluster = DefaultCluster::new(default_cluster_url);

        Ok(Box::new(cluster))
    }
}


pub struct DefaultCluster {
    url: Url,
}

impl DefaultCluster {

    pub fn new(url: Url) -> Self {
        Self {
            url,
        }
    }
}


#[async_trait]
impl Cluster for DefaultCluster {

    async fn join(&mut self, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError> {
        
        let invoker = DefaultClusterInvoker::new(self.url.clone(), invokers);

        Ok(Box::new(invoker))
    }

}


pub struct DefaultClusterInvoker {
    url: Url,
    invokers: Vec<Box<dyn Invoker + Send>>,
}

impl DefaultClusterInvoker {

    pub const NAME: &'static str = "default-cluster-invoker";

    pub fn new(mut url: Url, mut invokers: Vec<Box<dyn Invoker + Send>>) -> Self {
        let mut rng = rand::thread_rng();
        invokers.shuffle(&mut rng);

        url.set_protocol(DefaultClusterInvoker::NAME);
        Self {
            url,
            invokers,
        }
    }
}



#[async_trait::async_trait]
impl Invoker for DefaultClusterInvoker {

    async fn ready(&mut self) -> Result<(), StdError> {

        Ok(())
    }

    async fn invoke(&mut self, invocation: RpcInvocation) -> Result<RpcResponse, StdError> {
        match self.invokers.first_mut() {
            Some(invoker) => {
                invoker.invoke(invocation).await
            },
            None => {
                Err(NoAvailableInvokerError::new("no available invoker").into())
            },
        }
    }

    fn url(&self) -> &Url {
        &self.url
    }
}


#[derive(Error, Debug)]
#[error("no available invoker")]
pub struct NoAvailableInvokerError(String);

impl NoAvailableInvokerError {

    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
    
}