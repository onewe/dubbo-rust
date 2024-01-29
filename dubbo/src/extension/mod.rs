use std::{collections::HashMap, str::FromStr};

use self::{cluster_extension::{proxy::ClusterProxy, Cluster}, directory_extension::{proxy::InvokerDirectoryProxy, InvokerDirectory}, load_balance_extension::{proxy::LoadBalanceProxy, LoadBalance}, protocol_extension::{proxy::ProtocolProxy, Protocol}, registry_extension::{proxy::RegistryProxy, Registry}, router_extension::{proxy::RouterProxy, Router}};
use crate::StdError;
use dubbo_base::{url::UrlParam, Url};
use tracing::{debug, error};
use thiserror::Error;
use tokio::sync::oneshot;

pub mod registry_extension;
pub mod protocol_extension;
pub mod cluster_extension;
pub mod load_balance_extension;
pub mod router_extension;
pub mod directory_extension;

pub(crate) static INSTANCE: once_cell::sync::Lazy<ExtensionDirectoryCommander> = once_cell::sync::Lazy::new(|| ExtensionDirectory::init());

#[derive(Default)]
struct ExtensionDirectory {
    protocol_extension_loaders: HashMap<String, ProtocolExtensionLoaderWrapper>,
    registry_extension_loaders: HashMap<String, RegistryExtensionLoaderWrapper>,
    cluster_extension_loaders: HashMap<String, ClusterExtensionLoaderWrapper>,
    load_balance_extension_loaders: HashMap<String, LoadBalanceExtensionLoaderWrapper>,
    router_extension_loaders: HashMap<String, RouterExtensionLoaderWrapper>,
    directory_extension_loaders: HashMap<String, InvokerDirectoryExtensionLoaderWrapper>,
}

impl ExtensionDirectory {

    fn init() -> ExtensionDirectoryCommander {
        let (tx, rx) = tokio::sync::mpsc::channel::<ExtensionOpt>(64);

        let directory = ExtensionDirectory::default();

        tokio::spawn(async move {
            directory.run(rx).await;
        });
        
        ExtensionDirectoryCommander::new(tx)
    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::Receiver<ExtensionOpt>) {
        while let Some(opt) = rx.recv().await {
            match opt {
             ExtensionOpt::AddClusterExtensionLoader(loader) => self.add_cluster_extension_loader(loader),
             ExtensionOpt::AddLoadBalanceExtensionLoader(loader) => self.add_load_balance_extension_loader(loader),
             ExtensionOpt::AddProtocolExtensionLoader(loader) => self.add_protocol_extension_loader(loader),
             ExtensionOpt::AddRegistryExtensionLoader(loader) => self.add_registry_extension_loader(loader),
             ExtensionOpt::AddRouterExtensionLoader(loader) => self.add_router_extension_loader(loader),
             ExtensionOpt::AddInvokerDirectoryExtensionLoader(loader) => self.add_invoker_directory_extension_loader(loader),

             ExtensionOpt::RemoveClusterExtensionLoader(name) => self.remove_cluster_extension_loader(&name),
             ExtensionOpt::RemoveLoadBalanceExtensionLoader(name) => self.remove_load_balance_extension_loader(&name),
             ExtensionOpt::RemoveProtocolExtensionLoader(name) => self.remove_protocol_extension_loader(&name),
             ExtensionOpt::RemoveRegistryExtensionLoader(name) => self.remove_registry_extension_loader(&name),
             ExtensionOpt::RemoveRouterExtensionLoader(name) => self.remove_router_extension_loader(&name),
             ExtensionOpt::RemoveInvokerDirectoryExtensionLoader(name) => self.remove_invoker_directory_extension_loader(&name),
 
 
             ExtensionOpt::LoadProtocolExtension(url, tx) => self.load_protocol_extension(url, tx).await,
             ExtensionOpt::LoadRegistryExtension(url, tx) => self.load_registry_extension(url, tx).await,
             ExtensionOpt::LoadClusterExtension(url, tx) => self.load_cluster_extension(url, tx).await,
             ExtensionOpt::LoadLoadBalanceExtension(url, tx) => self.load_load_balance_extension(url, tx).await,
             ExtensionOpt::LoadRouterExtension(url, tx) => self.load_router_extension(url, tx).await,
             ExtensionOpt::LoadInvokerDirectoryExtension(url, tx) => self.load_invoker_directory_extension(url, tx).await,
            }
        }
    }

    fn add_protocol_extension_loader(&mut self, loader: Box<dyn ProtocolExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add protocol extension loader, name: {}", name);
        self.protocol_extension_loaders.insert(name, ProtocolExtensionLoaderWrapper::new(loader));
    }

    fn add_registry_extension_loader(&mut self, loader: Box<dyn RegistryExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add registry extension loader, name: {}", name);
        self.registry_extension_loaders.insert(name, RegistryExtensionLoaderWrapper::new(loader));
    }

    fn add_cluster_extension_loader(&mut self, loader: Box<dyn ClusterExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add cluster extension loader, name: {}", name);
        self.cluster_extension_loaders.insert(name, ClusterExtensionLoaderWrapper::new(loader));
    }

    fn add_load_balance_extension_loader(&mut self, loader: Box<dyn LoadBalanceExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add load balance extension loader, name: {}", name);
        self.load_balance_extension_loaders.insert(name, LoadBalanceExtensionLoaderWrapper::new(loader));
    }

    fn add_router_extension_loader(&mut self, loader: Box<dyn RouterExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add router extension loader, name: {}", name);
        self.router_extension_loaders.insert(name, RouterExtensionLoaderWrapper::new(loader));
    }

    fn add_invoker_directory_extension_loader(&mut self, loader: Box<dyn InvokerDirectoryExtensionLoader + Send>) {
        let name = loader.name();
        debug!("add invoker directory extension loader, name: {}", name);
        self.directory_extension_loaders.insert(name, InvokerDirectoryExtensionLoaderWrapper::new(loader));
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

    fn remove_invoker_directory_extension_loader(&mut self, name: &str) {
        debug!("remove invoker directory extension loader, name: {}", name);
        self.directory_extension_loaders.remove(name);
    }


    async fn load_protocol_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn Protocol + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.protocol_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load protocol extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load protocol extension error: {}", err);
                        },
                    }
                } else {
                    error!("load protocol extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load protocol extension error: extension name not found, url: {}", url);
            },
        }
    }

    async fn load_registry_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn Registry + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.registry_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load registry extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load registry extension error: {}", err);
                        },
                    }
                } else {
                    error!("load registry extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load registry extension error: extension name not found, url: {}", url);
            },
        }
    }

    async fn load_cluster_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn Cluster + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.cluster_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load cluster extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load cluster extension error: {}", err);
                        },
                    }
                } else {
                    error!("load cluster extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load cluster extension error: extension name not found, url: {}", url);
            },
        }
    }

    async fn load_load_balance_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn LoadBalance + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.load_balance_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load load balance extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load load balance extension error: {}", err);
                        },
                    }
                } else {
                    error!("load load balance extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load load balance extension error: extension name not found, url: {}", url);
            },
        }
    }


    async fn load_router_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn Router + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.router_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load router extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load router extension error: {}", err);
                        },
                    }
                } else {
                    error!("load router extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load router extension error: extension name not found, url: {}", url);
            },
        }
    }

    async fn load_invoker_directory_extension(&mut self, url: Url, tx: oneshot::Sender<Box<dyn InvokerDirectory + Send>>) {
        let extension_loader_name = url.query::<ExtensionLoaderName>();
        match extension_loader_name {
            Some(extension_loader_name) => {
                let extension_loader_name = extension_loader_name.value();
                if let Some(loader) = self.directory_extension_loaders.get_mut(&extension_loader_name) {
                    match loader.load(&url).await {
                        Ok(extension) => {
                            match tx.send(extension) {
                                Ok(_) => {},
                                Err(_) => {
                                    error!("load invoker directory extension error: send load extension response failed, url: {}", url);
                                },
                            }
                        },
                        Err(err) => {
                            error!("load invoker directory extension error: {}", err);
                        },
                    }
                } else {
                    error!("load invoker directory extension error: extension loader not found, name: {}", extension_loader_name);
                }
            },
            None => {
                error!("load invoker directory extension error: extension name not found, url: {}", url);
            },
        }
    }

}


pub(crate) struct ExtensionDirectoryCommander {
    sender: tokio::sync::mpsc::Sender<ExtensionOpt>,
}



impl ExtensionDirectoryCommander {

    fn new(sender: tokio::sync::mpsc::Sender<ExtensionOpt>) -> Self {
        ExtensionDirectoryCommander {
            sender,
        }
    }

    pub(crate) async fn add_protocol_extension_loader(&self, loader: Box<dyn ProtocolExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddProtocolExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add protocol extension loader failed").into()),
        }
    }

    pub(crate) async fn add_registry_extension_loader(&self, loader: Box<dyn RegistryExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRegistryExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add registry extension loader failed").into()),
        }
    }

    pub(crate) async fn add_cluster_extension_loader(&self, loader: Box<dyn ClusterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddClusterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add cluster extension loader failed").into()),
        }
    }

    pub(crate) async fn add_load_balance_extension_loader(&self, loader: Box<dyn LoadBalanceExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddLoadBalanceExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add load balance extension loader failed").into()),
        }
    }

    pub(crate) async fn add_router_extension_loader(&self, loader: Box<dyn RouterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRouterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add router extension loader failed").into()),
        }
    }

    pub(crate) async fn add_invoker_directory_extension_loader(&self, loader: Box<dyn InvokerDirectoryExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddInvokerDirectoryExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add invoker directory extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_protocol_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveProtocolExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove protocol extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_registry_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRegistryExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove registry extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_cluster_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveClusterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove cluster extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_load_balance_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveLoadBalanceExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove load balance extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_router_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRouterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove router extension loader failed").into()),
        }
    }

    pub(crate) async fn remove_invoker_directory_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveInvokerDirectoryExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove invoker directory extension loader failed").into()),
        }
    }

    pub(crate) async fn load_protocol_extension(&self, url: Url) -> Result<Box<dyn Protocol + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadProtocolExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load protocol extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load protocol extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }
    
    pub(crate) async fn load_registry_extension(&self, url: Url) -> Result<Box<dyn Registry + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadRegistryExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load registry extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load registry extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub(crate) async fn load_cluster_extension(&self, url: Url) -> Result<Box<dyn Cluster + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadClusterExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load cluster extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load cluster extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub(crate) async fn load_load_balance_extension(&self, url: Url) -> Result<Box<dyn LoadBalance + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadLoadBalanceExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load load balance extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load load balance extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub(crate) async fn load_router_extension(&self, url: Url) -> Result<Box<dyn Router + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadRouterExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load router extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load router extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

    pub(crate) async fn load_invoker_directory_extension(&self, url: Url) -> Result<Box<dyn InvokerDirectory + Send>, StdError> {
        let (tx, rx) = oneshot::channel();
        match self.sender.send(ExtensionOpt::LoadInvokerDirectoryExtension(url.clone(), tx)).await {
            Ok(_) => {
                match rx.await {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        error!("load invoker directory extension error: receive load extension response failed, url: {}", url);
                        return Err(LoadExtensionError::new("receive load extension response failed").into());
                    },
                }
            },
            Err(_) => {
                error!("load invoker directory extension error: send load extension request failed, url: {}", url);
                return Err(LoadExtensionError::new("send load extension request failed").into());
            },
        }
    }

}


enum ExtensionOpt {
    AddProtocolExtensionLoader(Box<dyn ProtocolExtensionLoader + Send>),
    AddRegistryExtensionLoader(Box<dyn RegistryExtensionLoader + Send>),
    AddClusterExtensionLoader(Box<dyn ClusterExtensionLoader + Send>),
    AddLoadBalanceExtensionLoader(Box<dyn LoadBalanceExtensionLoader + Send>),
    AddRouterExtensionLoader(Box<dyn RouterExtensionLoader + Send>),
    AddInvokerDirectoryExtensionLoader(Box<dyn InvokerDirectoryExtensionLoader + Send>),

    RemoveProtocolExtensionLoader(String),
    RemoveRegistryExtensionLoader(String),
    RemoveClusterExtensionLoader(String),
    RemoveLoadBalanceExtensionLoader(String),
    RemoveRouterExtensionLoader(String),
    RemoveInvokerDirectoryExtensionLoader(String),

    LoadProtocolExtension(Url, oneshot::Sender<Box<dyn Protocol + Send>>),
    LoadRegistryExtension(Url, oneshot::Sender<Box<dyn Registry + Send>>),
    LoadClusterExtension(Url,  oneshot::Sender<Box<dyn Cluster + Send>>),
    LoadLoadBalanceExtension(Url, oneshot::Sender<Box<dyn LoadBalance + Send>>),
    LoadRouterExtension(Url,  oneshot::Sender<Box<dyn Router + Send>>),
    LoadInvokerDirectoryExtension(Url, oneshot::Sender<Box<dyn InvokerDirectory + Send>>),
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



macro_rules! extension_loader {
    ($name:ident<$extension_type:tt>) => {
        #[async_trait::async_trait]
        pub trait $name {
            fn name(&self) -> String;

            async fn load(&mut self, url: &Url) -> Result<Box<dyn $extension_type + Send>, StdError>;
        }
    };
}


extension_loader!(ProtocolExtensionLoader<Protocol>);

extension_loader!(RegistryExtensionLoader<Registry>);

extension_loader!(ClusterExtensionLoader<Cluster>);

extension_loader!(LoadBalanceExtensionLoader<LoadBalance>);

extension_loader!(RouterExtensionLoader<Router>);

extension_loader!(InvokerDirectoryExtensionLoader<InvokerDirectory>);



macro_rules! extension_loader_wrapper {

    ($loader_wrapper:ident[$loader:ident<$extension_type:tt<=>$proxy_type:tt>]) => {
        
        extension_loader_wrapper!($loader_wrapper, $loader, $extension_type, $proxy_type);
    };
    ($loader_wrapper:ident, $loader:ident, $extension_type:tt, $proxy_type:tt) => {
            
        struct $loader_wrapper {
            loader: Box<dyn $loader + Send>,
            extensions: HashMap<String, $proxy_type>,
        }

        impl $loader_wrapper {

            fn new(loader: Box<dyn $loader + Send>) -> Self {
                $loader_wrapper {
                    loader,
                    extensions: HashMap::new(),
                }
            }
        }

        #[async_trait::async_trait]
        impl $loader for $loader_wrapper {

            fn name(&self) -> String {
                self.loader.name()
            }

            async fn load(&mut self, url: &Url) -> Result<Box<dyn $extension_type + Send>, StdError> {
                let extension_name = url.query::<ExtensionName>();
                let extension_name = match extension_name {
                    None => "default".to_string(),
                    Some(extension_name) => extension_name.value(),
                };

                if let Some(extension_proxy) = self.extensions.get(&extension_name) {
                    return Ok(Box::new(extension_proxy.clone()));
                }

                let extension = self.loader.load(url).await?;
                let extension = $proxy_type::from(extension);
                self.extensions.insert(extension_name, extension.clone());
                Ok(Box::new(extension))
            }
        }

    };
}

extension_loader_wrapper!(ProtocolExtensionLoaderWrapper[ProtocolExtensionLoader<Protocol<=>ProtocolProxy>]);
extension_loader_wrapper!(RegistryExtensionLoaderWrapper[RegistryExtensionLoader<Registry<=>RegistryProxy>]);
extension_loader_wrapper!(ClusterExtensionLoaderWrapper[ClusterExtensionLoader<Cluster<=>ClusterProxy>]);
extension_loader_wrapper!(LoadBalanceExtensionLoaderWrapper[LoadBalanceExtensionLoader<LoadBalance<=>LoadBalanceProxy>]);
extension_loader_wrapper!(RouterExtensionLoaderWrapper[RouterExtensionLoader<Router<=>RouterProxy>]);
extension_loader_wrapper!(InvokerDirectoryExtensionLoaderWrapper[InvokerDirectoryExtensionLoader<InvokerDirectory<=>InvokerDirectoryProxy>]);



pub fn build_extension_loader_url(host: &str, extension_loader_name: &str, extension_name: &str) -> Url {
    let mut extension_url = Url::empty();

    extension_url.set_protocol("extensions");
    extension_url.set_host(host);
    extension_url.add_query_param(ExtensionLoaderName::new(extension_loader_name));
    extension_url.add_query_param(ExtensionName::new(extension_name));

    extension_url
}


struct ExtensionLoaderName(String);

impl ExtensionLoaderName {
   fn new(name: &str) -> Self {
        ExtensionLoaderName(name.to_string())
    }
}


impl UrlParam for ExtensionLoaderName {

    type TargetType = String;

    fn name() -> &'static str {
        "extension-loader-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
    
}

impl FromStr for ExtensionLoaderName {
    
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ExtensionLoaderName::new(s))
    }
    
}

struct ExtensionName(String);

impl ExtensionName {

    fn new(name: &str) -> Self {
        ExtensionName(name.to_string())
    }
}

impl UrlParam for ExtensionName {

    type TargetType = String;

    fn name() -> &'static str {
        "extension-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for ExtensionName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ExtensionName::new(s))
    }
}