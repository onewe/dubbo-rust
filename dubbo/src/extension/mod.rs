use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::Stream;
use thiserror::Error;
use tokio::sync::{mpsc::Receiver, watch, oneshot, Notify};
use tower::discover::Change;
use tracing::{error, warn, debug};

use crate::{url::Url, StdError, inv::Invoker};

use self::{registry_extension::Registry, protocol_extension::Protocol, cluster_extension::Cluster, load_balance_extension::LoadBalance, router_extension::Router};

mod registry_extension;
mod protocol_extension;
mod cluster_extension;
mod load_balance_extension;
mod router_extension;

#[derive(Default)]
pub struct ExtensionDirectory {

    protocol_extension_loaders: HashMap<String, Box<dyn ProtocolExtensionLoader>>,

    registry_extension_loaders: HashMap<String, Box<dyn RegistryExtensionLoader>>,

    cluster_extension_loaders: HashMap<String, Box<dyn ClusterExtensionLoader>>,

    load_balance_extension_loaders: HashMap<String, Box<dyn LoadBalanceExtensionLoader>>,

    router_extension_loaders: HashMap<String, Box<dyn RouterExtensionLoader>>,
}


impl ExtensionDirectory {

    pub fn init() -> ExtensionDirectoryCommander {

        let (tx, rx) = tokio::sync::mpsc::channel::<ExtensionOpt>(64);

        let mut directory = ExtensionDirectory::default();

        
        todo!()

    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::Receiver<ExtensionOpt>) {
       while let Some(opt) = rx.recv().await {
           match opt {
            ExtensionOpt::AddClusterExtensionLoader(loader) => self.add_cluster_extension_loader(loader),
            ExtensionOpt::AddLoadBalanceExtensionLoader(loader) => self.add_load_balance_extension_loader(loader),
            ExtensionOpt::AddProtocolExtensionLoader(loader) => self.add_protocol_extension_loader(loader),
            ExtensionOpt::AddRegistryExtensionLoader(loader) => self.add_registry_extension_loader(loader),
            ExtensionOpt::AddRouterExtensionLoader(loader) => self.add_router_extension_loader(loader),

            ExtensionOpt::RemoveClusterExtensionLoader(name) => self.remove_cluster_extension_loader(&name),
            ExtensionOpt::RemoveLoadBalanceExtensionLoader(name) => self.remove_load_balance_extension_loader(&name),
            ExtensionOpt::RemoveProtocolExtensionLoader(name) => self.remove_protocol_extension_loader(&name),
            ExtensionOpt::RemoveRegistryExtensionLoader(name) => self.remove_registry_extension_loader(&name),
            ExtensionOpt::RemoveRouterExtensionLoader(name) => self.remove_router_extension_loader(&name),

            _ => {}
           }
       }
    }

    fn add_protocol_extension_loader(&mut self, loader: Box<dyn ProtocolExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add protocol extension loader, name: {}", name);
        self.protocol_extension_loaders.insert(name, loader);
    }

    fn add_registry_extension_loader(&mut self, loader: Box<dyn RegistryExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add registry extension loader, name: {}", name);
        self.registry_extension_loaders.insert(name, loader);
    }

    fn add_cluster_extension_loader(&mut self, loader: Box<dyn ClusterExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add cluster extension loader, name: {}", name);
        self.cluster_extension_loaders.insert(name, loader);
    }

    fn add_load_balance_extension_loader(&mut self, loader: Box<dyn LoadBalanceExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add load balance extension loader, name: {}", name);
        self.load_balance_extension_loaders.insert(name, loader);
    }

    fn add_router_extension_loader(&mut self, loader: Box<dyn RouterExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add router extension loader, name: {}", name);
        self.router_extension_loaders.insert(name, loader);
    }

    fn remove_protocol_extension_loader(&mut self, name: &str) {
        debug!("remove protocol extension loader, name: {}", name);
        self.protocol_extension_loaders.remove(name);
    }

    fn remove_registry_extension_loader(&mut self, name: &str) {
        debug!("remove registry extension loader, name: {}", name);
        self.registry_extension_loaders.remove(name);
    }

    fn remove_cluster_extension_loader(&mut self, name: &str) {
        debug!("remove cluster extension loader, name: {}", name);
        self.cluster_extension_loaders.remove(name);
    }

    fn remove_load_balance_extension_loader(&mut self, name: &str) {
        debug!("remove load balance extension loader, name: {}", name);
        self.load_balance_extension_loaders.remove(name);
    }

    fn remove_router_extension_loader(&mut self, name: &str) {
        debug!("remove router extension loader, name: {}", name);
        self.router_extension_loaders.remove(name);
    }

    async fn load_protocol_extension(&mut self, name: &str, url: Url, tx: oneshot::Sender<protocol_extension::proxy::ProtocolProxy>) {
        debug!("load protocol extension, name: {}, url: {}", name, url);
        
        
    }

}


pub struct ExtensionDirectoryCommander {
    sender: tokio::sync::mpsc::Sender<ExtensionOpt>,
}

impl ExtensionDirectoryCommander {

    pub fn new(sender: tokio::sync::mpsc::Sender<ExtensionOpt>) -> Self {
        ExtensionDirectoryCommander {
            sender,
        }
    }

    pub async fn add_protocol_extension_loader(&self, loader: Box<dyn ProtocolExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddProtocolExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add protocol extension loader failed").into()),
        }
    }

    pub async fn add_registry_extension_loader(&self, loader: Box<dyn RegistryExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRegistryExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add registry extension loader failed").into()),
        }
    }

    pub async fn add_cluster_extension_loader(&self, loader: Box<dyn ClusterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddClusterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add cluster extension loader failed").into()),
        }
    }

    pub async fn add_load_balance_extension_loader(&self, loader: Box<dyn LoadBalanceExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddLoadBalanceExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add load balance extension loader failed").into()),
        }
    }

    pub async fn add_router_extension_loader(&self, loader: Box<dyn RouterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRouterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add router extension loader failed").into()),
        }
    }

    pub async fn remove_protocol_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveProtocolExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove protocol extension loader failed").into()),
        }
    }

    pub async fn remove_registry_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRegistryExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove registry extension loader failed").into()),
        }
    }

    pub async fn remove_cluster_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveClusterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove cluster extension loader failed").into()),
        }
    }

    pub async fn remove_load_balance_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveLoadBalanceExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove load balance extension loader failed").into()),
        }
    }

    pub async fn remove_router_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRouterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove router extension loader failed").into()),
        }
    }

    pub async fn load_protocol_extension(&self, name: &str, url: Url) -> Result<protocol_extension::proxy::ProtocolProxy, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadProtocolExtension(name.to_string(), url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load protocol extension error: receive load extension response failed, name: {}, url: {}", name, url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load protocol extension error: send load extension request failed, name: {}, url: {}", name, url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }
    
    pub async fn load_registry_extension(&self, name: &str, url: Url) -> Result<registry_extension::proxy::RegistryProxy, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadRegistryExtension(name.to_string(), url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load registry extension error: receive load extension response failed, name: {}, url: {}", name, url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load registry extension error: send load extension request failed, name: {}, url: {}", name, url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub async fn load_cluster_extension(&self, name: &str, url: Url, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<cluster_extension::proxy::ClusterProxy, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadClusterExtension(name.to_string(), url.clone(), invokers, tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load cluster extension error: receive load extension response failed, name: {}, url: {}", name, url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load cluster extension error: send load extension request failed, name: {}, url: {}", name, url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub async fn load_load_balance_extension(&self, name: &str, url: Url, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<load_balance_extension::proxy::LoadBalanceProxy, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadLoadBalanceExtension(name.to_string(), url.clone(), invokers, tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load load balance extension error: receive load extension response failed, name: {}, url: {}", name, url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load load balance extension error: send load extension request failed, name: {}, url: {}", name, url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub async fn load_router_extension(&self, name: &str, url: Url, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<router_extension::proxy::RouterProxy, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadRouterExtension(name.to_string(), url.clone(), invokers, tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load router extension error: receive load extension response failed, name: {}, url: {}", name, url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load router extension error: send load extension request failed, name: {}, url: {}", name, url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct AddExtensionLoaderError(String);

impl AddExtensionLoaderError {
    
    pub fn new(msg: &str) -> Self {
        AddExtensionLoaderError(msg.to_string())
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct RemoveExtensionLoaderError(String);

impl RemoveExtensionLoaderError {
    
    pub fn new(msg: &str) -> Self {
        RemoveExtensionLoaderError(msg.to_string())
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct LoadExtensionError(String);

impl LoadExtensionError {
    
    pub fn new(msg: &str) -> Self {
        LoadExtensionError(msg.to_string())
    }
}



pub enum ExtensionOpt {
    AddProtocolExtensionLoader(Box<dyn ProtocolExtensionLoader + Send>),
    AddRegistryExtensionLoader(Box<dyn RegistryExtensionLoader + Send>),
    AddClusterExtensionLoader(Box<dyn ClusterExtensionLoader + Send>),
    AddLoadBalanceExtensionLoader(Box<dyn LoadBalanceExtensionLoader + Send>),
    AddRouterExtensionLoader(Box<dyn RouterExtensionLoader + Send>),

    RemoveProtocolExtensionLoader(String),
    RemoveRegistryExtensionLoader(String),
    RemoveClusterExtensionLoader(String),
    RemoveLoadBalanceExtensionLoader(String),
    RemoveRouterExtensionLoader(String),

    LoadProtocolExtension(String, Url, oneshot::Sender<protocol_extension::proxy::ProtocolProxy>),
    LoadRegistryExtension(String, Url, oneshot::Sender<registry_extension::proxy::RegistryProxy>),
    LoadClusterExtension(String, Url, Vec<Box<dyn Invoker + Send>>, oneshot::Sender<cluster_extension::proxy::ClusterProxy>),
    LoadLoadBalanceExtension(String, Url, Vec<Box<dyn Invoker + Send>>, oneshot::Sender<load_balance_extension::proxy::LoadBalanceProxy>),
    LoadRouterExtension(String, Url, Vec<Box<dyn Invoker + Send>>, oneshot::Sender<router_extension::proxy::RouterProxy>),
}




macro_rules! extension_loader {
    ($name:ident<$extension_type:tt>) => {
        #[async_trait::async_trait]
        pub trait $name {
            fn name(&self) -> String;

            async fn load(&mut self, url: &Url) -> Result<Box<dyn $extension_type>, StdError>;
        }
    };
}


extension_loader!(ProtocolExtensionLoader<Protocol>);

extension_loader!(RegistryExtensionLoader<Registry>);

extension_loader!(ClusterExtensionLoader<Cluster>);

extension_loader!(LoadBalanceExtensionLoader<LoadBalance>);

extension_loader!(RouterExtensionLoader<Router>);