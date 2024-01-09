use async_trait::async_trait;
use futures::Stream;
use tokio::sync::{mpsc::Receiver, watch};
use tower::discover::Change;

use crate::{url::Url, StdError, inv::Invoker};

#[derive(Default)]
pub struct ExtensionDirectory {

    protocol_extension_loaders: Vec<Box<dyn ProtocolExtensionLoader>>,

    registry_extension_loaders: Vec<Box<dyn RegistryExtensionLoader>>,

    cluster_extension_loaders: Vec<Box<dyn ClusterExtensionLoader>>,

    load_balance_extension_loaders: Vec<Box<dyn LoadBalanceExtensionLoader>>,

    router_extension_loaders: Vec<Box<dyn RouterExtensionLoader>>,
}


impl ExtensionDirectory {

    pub fn new() -> Self {
        Self {
            protocol_extension_loaders: Vec::new(),
            registry_extension_loaders: Vec::new(),
            cluster_extension_loaders: Vec::new(),
            load_balance_extension_loaders: Vec::new(),
            router_extension_loaders: Vec::new(),
        }
    }
}


impl ExtensionDirectory {

    pub fn add_protocol_extension_loader(&mut self, loader: Box<dyn ProtocolExtensionLoader>) {
        self.protocol_extension_loaders.push(loader);
    }

    pub fn add_registry_extension_loader(&mut self, loader: Box<dyn RegistryExtensionLoader>) {
        self.registry_extension_loaders.push(loader);
    }

    pub fn add_cluster_extension_loader(&mut self, loader: Box<dyn ClusterExtensionLoader>) {
        self.cluster_extension_loaders.push(loader);
    }

    pub fn add_load_balance_extension_loader(&mut self, loader: Box<dyn LoadBalanceExtensionLoader>) {
        self.load_balance_extension_loaders.push(loader);
    }

    pub fn add_router_extension_loader(&mut self, loader: Box<dyn RouterExtensionLoader>) {
        self.router_extension_loaders.push(loader);
    }

    pub async fn find_protocol_extension_loader(&mut self, url: &Url) -> Option<&mut dyn ProtocolExtensionLoader> {
        for loader in self.protocol_extension_loaders.iter_mut() {
            if loader.support(url).await {
               return Some(loader.as_mut());
            }
        }
        None
    }

    pub async fn find_registry_extension_loader(&mut self, url: &Url) -> Option<&mut dyn RegistryExtensionLoader> {
        for loader in self.registry_extension_loaders.iter_mut() {
            if loader.support(url).await {
               return Some(loader.as_mut());
            }
        }
        None
    }

}


#[async_trait]
pub trait Registry {

    async fn register(&mut self, url: &Url) -> Result<(), StdError>;

    async fn unregister(&mut self, url: &Url) -> Result<(), StdError>;

    async fn subscribe(&mut self, url: &Url) -> Result<Box<dyn Stream<Item = Vec<String>> + Send>, StdError>;

}

#[async_trait]
pub trait Protocol {
    
    async fn export(&mut self, url: &Url) -> Result<(), StdError>;

    async fn refer(&mut self, url: &Url) -> Result<Box<dyn Invoker>, StdError>;
}


#[async_trait]
pub trait Cluster {

    async fn join(&self, url: &Url, invokers: Vec<Box<dyn Invoker>>) -> Result<Box<dyn Invoker>, StdError>;
    
}


#[async_trait]
pub trait LoadBalance {

    async fn select(&self, invokes: Vec<Box<dyn Invoker>>) -> Result<Box<dyn Invoker>, StdError>;
    
}

#[async_trait]
pub trait Router {
    
    async fn route(&self, invokes: Vec<Box<dyn Invoker>>) -> Result<Vec<Box<dyn Invoker>>, StdError>;
}

macro_rules! extension_loader {
    ($name:ident<$extension_type:tt>) => {
        #[async_trait::async_trait]
        pub trait $name {
            async fn support(&self, url: &Url) -> bool;

            async fn load(&mut self, url: &Url) -> Result<&mut dyn $extension_type, StdError>;
        }
    };
}


extension_loader!(ProtocolExtensionLoader<Protocol>);

extension_loader!(RegistryExtensionLoader<Registry>);

extension_loader!(ClusterExtensionLoader<Cluster>);

extension_loader!(LoadBalanceExtensionLoader<LoadBalance>);

extension_loader!(RouterExtensionLoader<Router>);