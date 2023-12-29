use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use tower::discover::Change;

use crate::{url::Url, StdError, inv::Invoker};

#[derive(Default)]
pub(crate) struct ExtensionDirectory {

    protocol_extension_loaders: Vec<Box<dyn ProtocolExtensionLoader>>,

    registry_extension_loaders: Vec<Box<dyn RegistryExtensionLoader>>,

    cluster_extension_loaders: Vec<Box<dyn ClusterExtensionLoader>>,

    directory_extension_loaders: Vec<Box<dyn DirectoryExtensionLoader>>,

    load_balance_extension_loaders: Vec<Box<dyn LoadBalanceExtensionLoader>>,

    router_extension_loaders: Vec<Box<dyn RouterExtensionLoader>>,
}


impl ExtensionDirectory {

    pub(crate) fn new() -> Self {
        Self {
            protocol_extension_loaders: Vec::new(),
            registry_extension_loaders: Vec::new(),
            cluster_extension_loaders: Vec::new(),
            directory_extension_loaders: Vec::new(),
            load_balance_extension_loaders: Vec::new(),
            router_extension_loaders: Vec::new(),
        }
    }
}


impl ExtensionDirectory {

    pub(crate) fn add_protocol_extension_loader(&mut self, loader: Box<dyn ProtocolExtensionLoader>) {
        self.protocol_extension_loaders.push(loader);
    }

    pub(crate) fn add_registry_extension_loader(&mut self, loader: Box<dyn RegistryExtensionLoader>) {
        self.registry_extension_loaders.push(loader);
    }

    pub(crate) fn add_cluster_extension_loader(&mut self, loader: Box<dyn ClusterExtensionLoader>) {
        self.cluster_extension_loaders.push(loader);
    }

    pub(crate) fn add_directory_extension_loader(&mut self, loader: Box<dyn DirectoryExtensionLoader>) {
        self.directory_extension_loaders.push(loader);
    }

    pub(crate) fn add_load_balance_extension_loader(&mut self, loader: Box<dyn LoadBalanceExtensionLoader>) {
        self.load_balance_extension_loaders.push(loader);
    }

    pub(crate) fn add_router_extension_loader(&mut self, loader: Box<dyn RouterExtensionLoader>) {
        self.router_extension_loaders.push(loader);
    }

}


pub(crate) trait ExtensionLoader {

    type Extension;

    fn support(&self, url: &Url) -> bool;

    fn load(&self, url: &Url) -> Self::Extension;
}


pub type DiscoverStream = Receiver<Result<Change<String, ()>, StdError>>;

#[async_trait]
pub(crate) trait Registry {

    async fn register(&mut self, url: &Url) -> Result<(), StdError>;

    async fn unregister(&mut self, url: &Url) -> Result<(), StdError>;

    async fn subscribe(&mut self, url: &Url) -> Result<DiscoverStream, StdError>;

    async fn unsubscribe(&mut self, url: &Url) -> Result<(), StdError>;

}

#[async_trait]
pub(crate) trait Protocol {
    
    async fn export(&mut self, url: &Url) -> Result<(), StdError>;

    async fn refer(&mut self, url: &Url) -> Result<Box<dyn Invoker>, StdError>;
}


#[async_trait]
pub(crate) trait Cluster {

    async fn join(&mut self, url: &Url, invokers: Vec<Box<dyn Invoker>>) -> Result<Box<dyn Invoker>, StdError>;
    
}

#[async_trait]
pub(crate) trait Directory {
    
    async fn list(&mut self) -> Result<Vec<Box<dyn Invoker>>, StdError>;
}

#[async_trait]
pub(crate) trait LoadBalance {

    async fn select(&mut self, invokes: Vec<Box<dyn Invoker>>) -> Result<Box<dyn Invoker>, StdError>;
    
}

#[async_trait]
pub(crate) trait Router {
    
    async fn route(&mut self, invokes: Vec<Box<dyn Invoker>>) -> Result<Vec<Box<dyn Invoker>>, StdError>;
}

macro_rules! extension_loader {
    ($name:ident<$extension_type:tt>) => {
        #[async_trait::async_trait]
        pub(crate) trait $name {
            async fn support(&self, url: &Url) -> bool;

            async fn load(&mut self, url: &Url) -> Result<&mut dyn $extension_type, StdError>;
        }
    };
}


extension_loader!(ProtocolExtensionLoader<Protocol>);

extension_loader!(RegistryExtensionLoader<Registry>);

extension_loader!(ClusterExtensionLoader<Cluster>);

extension_loader!(DirectoryExtensionLoader<Directory>);

extension_loader!(LoadBalanceExtensionLoader<LoadBalance>);

extension_loader!(RouterExtensionLoader<Router>);