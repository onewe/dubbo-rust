use std::collections::HashMap;

use thiserror::Error;
use tokio::sync::oneshot;
use tracing::{error, debug};

use crate::{url::Url, StdError, param::{Extension, Param}, extension::{protocol_extension::proxy::ProtocolOpt, registry_extension::proxy::RegistryOpt}};

use self::{registry_extension::Registry, protocol_extension::Protocol, cluster_extension::Cluster, load_balance_extension::LoadBalance, router_extension::Router};

mod registry_extension;
mod protocol_extension;
mod cluster_extension;
mod load_balance_extension;
mod router_extension;

static INSTANCE: once_cell::sync::Lazy<ExtensionDirectoryCommander> = once_cell::sync::Lazy::new(|| ExtensionDirectory::init());

pub async fn add_protocol_extension_loader(loader: Box<dyn ProtocolExtensionLoader + Send>) -> Result<(), StdError> {
    INSTANCE.add_protocol_extension_loader(loader).await
}


pub async fn add_registry_extension_loader(loader: Box<dyn RegistryExtensionLoader + Send>) -> Result<(), StdError> {
    INSTANCE.add_registry_extension_loader(loader).await
}

pub async fn add_cluster_extension_loader(loader: Box<dyn ClusterExtensionLoader + Send>) -> Result<(), StdError> {
    INSTANCE.add_cluster_extension_loader(loader).await
}

pub async fn add_load_balance_extension_loader(loader: Box<dyn LoadBalanceExtensionLoader + Send>) -> Result<(), StdError> {
    INSTANCE.add_load_balance_extension_loader(loader).await
}

pub async fn add_router_extension_loader(loader: Box<dyn RouterExtensionLoader + Send>) -> Result<(), StdError> {
    INSTANCE.add_router_extension_loader(loader).await
}

pub async fn remove_protocol_extension_loader(name: &str) -> Result<(), StdError> {
    INSTANCE.remove_protocol_extension_loader(name).await
}

pub async fn remove_registry_extension_loader(name: &str) -> Result<(), StdError> {
    INSTANCE.remove_registry_extension_loader(name).await
}

pub async fn remove_cluster_extension_loader(name: &str) -> Result<(), StdError> {
    INSTANCE.remove_cluster_extension_loader(name).await
}

pub async fn remove_load_balance_extension_loader(name: &str) -> Result<(), StdError> {
    INSTANCE.remove_load_balance_extension_loader(name).await
}

pub async fn remove_router_extension_loader(name: &str) -> Result<(), StdError> {
    INSTANCE.remove_router_extension_loader(name).await
}

pub async fn load_protocol_extension(url: Url) -> Result<protocol_extension::proxy::ProtocolProxy, StdError> {
    INSTANCE.load_protocol_extension(url).await
}

pub async fn load_registry_extension(url: Url) -> Result<registry_extension::proxy::RegistryProxy, StdError> {
    INSTANCE.load_registry_extension(url).await
}

pub async fn load_cluster_extension(url: Url) -> Result<cluster_extension::proxy::ClusterProxy, StdError> {
    INSTANCE.load_cluster_extension(url).await
}

pub async fn load_load_balance_extension(url: Url) -> Result<load_balance_extension::proxy::LoadBalanceProxy, StdError> {
    INSTANCE.load_load_balance_extension(url).await
}

pub async fn load_router_extension(url: Url) -> Result<router_extension::proxy::RouterProxy, StdError> {
    INSTANCE.load_router_extension(url).await
}


#[derive(Default)]
struct ExtensionDirectory {

    protocol_extension_loaders: HashMap<String, Box<dyn ProtocolExtensionLoader + Send>>,

    protocol_extensions: HashMap<String, protocol_extension::proxy::ProtocolProxy>,

    registry_extension_loaders: HashMap<String, Box<dyn RegistryExtensionLoader + Send>>,

    registry_extensions: HashMap<String, registry_extension::proxy::RegistryProxy>,

    cluster_extension_loaders: HashMap<String, Box<dyn ClusterExtensionLoader + Send>>,

    cluster_extensions: HashMap<String, cluster_extension::proxy::ClusterProxy>,

    load_balance_extension_loaders: HashMap<String, Box<dyn LoadBalanceExtensionLoader + Send>>,

    load_balance_extensions: HashMap<String, load_balance_extension::proxy::LoadBalanceProxy>,

    router_extension_loaders: HashMap<String, Box<dyn RouterExtensionLoader + Send>>,

    router_extensions: HashMap<String, router_extension::proxy::RouterProxy>,
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

            ExtensionOpt::RemoveClusterExtensionLoader(name) => self.remove_cluster_extension_loader(&name),
            ExtensionOpt::RemoveLoadBalanceExtensionLoader(name) => self.remove_load_balance_extension_loader(&name),
            ExtensionOpt::RemoveProtocolExtensionLoader(name) => self.remove_protocol_extension_loader(&name),
            ExtensionOpt::RemoveRegistryExtensionLoader(name) => self.remove_registry_extension_loader(&name),
            ExtensionOpt::RemoveRouterExtensionLoader(name) => self.remove_router_extension_loader(&name),


            ExtensionOpt::LoadProtocolExtension(url, tx) => self.load_protocol_extension(url, tx).await,
            ExtensionOpt::LoadRegistryExtension(url, tx) => self.load_registry_extension(url, tx).await,
            ExtensionOpt::LoadClusterExtension(url, tx) => self.load_cluster_extension(url, tx).await,
            ExtensionOpt::LoadLoadBalanceExtension(url, tx) => self.load_load_balance_extension(url, tx).await,
            ExtensionOpt::LoadRouterExtension(url, tx) => self.load_router_extension(url, tx).await,
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

    async fn load_protocol_extension(&mut self, url: Url, tx: oneshot::Sender<protocol_extension::proxy::ProtocolProxy>) {
        debug!("load protocol extension, url: {}", url);
        let extension = url.query::<Extension>();
        match extension {
            None => {
                error!("load protocol extension error: extension not found, url: {}", url);
            },
            Some(extension) => {
                let name = extension.value();
                let extension_loader = self.protocol_extension_loaders.get_mut(&name);
                match extension_loader {
                    None => {
                        error!("load protocol extension error: extension loader not found, url: {}", url);
                    },
                    Some(extension_loader) => {
                        let extension_cache_key = url.to_string();
                        let protocol_extension = self.protocol_extensions.get(&extension_cache_key);
                        match protocol_extension {
                            None => {
                                match extension_loader.load(&url).await {
                                    Ok(mut protocol_extension) => {
                                        let (extension_opt_tx, mut extension_opt_rx) = tokio::sync::mpsc::channel(64);
                                        tokio::spawn(async move {
                                            while let Some(opt) = extension_opt_rx.recv().await {
                                                match opt {
                                                    ProtocolOpt::Export(url, callback) => {
                                                        let export = protocol_extension.export(url).await;
                                                        let _ = callback.send(export);
                                                    },
                                                    ProtocolOpt::Refer(url, callback) => {
                                                        let invoker = protocol_extension.refer(url).await;
                                                        let _ = callback.send(invoker);
                                                    },
                                                }
                                            }
                                            debug!("protocol extension destroy, url: {}", url);
                                        });

                                        let proxy = protocol_extension::proxy::ProtocolProxy::new(extension_opt_tx);
                                        self.protocol_extensions.insert(extension_cache_key, proxy.clone());
                                        let _ = tx.send(proxy);
                                    },
                                    Err(err) => {
                                        error!("load protocol extension error: load extension failed, url: {}, err: {}", url, err);
                                    },
                                }
                            },
                            Some(protocol_extension) => {
                                let _ = tx.send(protocol_extension.clone());
                            }
                        }
                    }
                }
            }
        }
        
    }


    async fn load_registry_extension(&mut self, url: Url, tx: oneshot::Sender<registry_extension::proxy::RegistryProxy>) {
        debug!("load registry extension, url: {}", url);
        let extension = url.query::<Extension>();
        match extension {
            None => {
                error!("load registry extension error: extension not found, url: {}", url);
            },
            Some(extension) => {
                let name = extension.value();
                match self.registry_extension_loaders.get_mut(&name) {
                    None => {
                        error!("load registry extension error: extension loader not found, url: {}", url);
                    },
                    Some(extension_loader) => {
                        let extension_cache_key = url.to_string();
                        match self.registry_extensions.get(&extension_cache_key) {
                            None => {
                                match extension_loader.load(&url).await {
                                    Ok(mut registry_extension) => {
                                        let (extension_opt_tx, mut extension_opt_rx) = tokio::sync::mpsc::channel(64);
                                        tokio::spawn(async move {
                                            while let Some(opt) = extension_opt_rx.recv().await {
                                                match opt {
                                                    RegistryOpt::Register(url, callback) => {
                                                        let register = registry_extension.register(url).await;
                                                        let _ = callback.send(register);
                                                    },
                                                    RegistryOpt::Unregister(url, callback) => {
                                                        let unregister = registry_extension.unregister(url).await;
                                                        let _ = callback.send(unregister);
                                                    },
                                                    RegistryOpt::Subscribe(url, callback) => {
                                                        let subscribe = registry_extension.subscribe(url).await;
                                                        let _ = callback.send(subscribe);
                                                    },
                                                }
                                            }
                                            debug!("registry extension destroy, url: {}", url);
                                        });

                                        let proxy = registry_extension::proxy::RegistryProxy::new(extension_opt_tx);
                                        self.registry_extensions.insert(extension_cache_key, proxy.clone());
                                        let _ = tx.send(proxy);
                                    },
                                    Err(err) => {
                                        error!("load registry extension error: load extension failed, url: {}, err: {}", url, err);
                                    },
                                }
                            },
                             Some(registry_extension) => {
                                let _ = tx.send(registry_extension.clone());
                             }
                        }
                    }
                }
            }

        }
        

    }


    async fn load_cluster_extension(&mut self, url: Url, tx: oneshot::Sender<cluster_extension::proxy::ClusterProxy>) {
        debug!("load cluster extension, url: {}", url);
        let extension = url.query::<Extension>();
        match extension {
            None => {
                error!("load cluster extension error: extension not found, url: {}", url);
            },
            Some(extension) => {
                let name = extension.value();
                match self.cluster_extension_loaders.get_mut(&name) {
                    None => {
                        error!("load cluster extension error: extension loader not found, url: {}", url);
                    },
                    Some(extension_loader) => {
                        let extension_cache_key = url.to_string();
                        match self.cluster_extensions.get(&extension_cache_key) {
                            None => {
                                match extension_loader.load(&url).await {
                                    Ok(mut cluster_extension) => {
                                        let (extension_opt_tx, mut extension_opt_rx) = tokio::sync::mpsc::channel(64);
                                        tokio::spawn(async move {
                                            while let Some(opt) = extension_opt_rx.recv().await {
                                                match opt {
                                                   cluster_extension::proxy::ClusterOpt::Join(url, invokes, callback) => {
                                                        let join = cluster_extension.join(url, invokes).await;
                                                        let _ = callback.send(join);
                                                    },
                                                }
                                            }
                                            debug!("cluster extension destroy, url: {}", url);
                                        });

                                        let proxy = cluster_extension::proxy::ClusterProxy::new(extension_opt_tx);
                                        self.cluster_extensions.insert(extension_cache_key, proxy.clone());
                                        let _ = tx.send(proxy);
                                    },
                                    Err(err) => {
                                        error!("load cluster extension error: load extension failed, url: {}, err: {}", url, err);
                                    },
                                }
                            },
                            Some(cluster_extension) => {
                                let _ = tx.send(cluster_extension.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    async fn load_load_balance_extension(&mut self, url: Url, tx: oneshot::Sender<load_balance_extension::proxy::LoadBalanceProxy>) {
        debug!("load load balance extension, url: {}", url);
        let extension = url.query::<Extension>();
        match extension {
            None => {
                error!("load load balance extension error: extension not found, url: {}", url);
            },
            Some(extension) => {
                let name = extension.value();
                match self.load_balance_extension_loaders.get_mut(&name) {
                    None => {
                        error!("load load balance extension error: extension loader not found, url: {}", url);
                    },
                    Some(extension_loader) => {
                        let extension_cache_key = url.to_string();
                        match self.load_balance_extensions.get(&extension_cache_key) {
                            None => {
                                match extension_loader.load(&url).await {
                                    Ok(mut load_balance_extension) => {
                                        let (extension_opt_tx, mut extension_opt_rx) = tokio::sync::mpsc::channel(64);
                                        tokio::spawn(async move {
                                            while let Some(opt) = extension_opt_rx.recv().await {
                                                match opt {
                                                    load_balance_extension::proxy::LoadBalanceOpt::Select(invokers, callback) => {
                                                        let select = load_balance_extension.select(invokers);
                                                        let _ = callback.send(select.await);
                                                    },
                                                }
                                            }
                                            debug!("load balance extension destroy, url: {}", url);
                                        });

                                        let proxy = load_balance_extension::proxy::LoadBalanceProxy::new(extension_opt_tx);
                                        self.load_balance_extensions.insert(extension_cache_key, proxy.clone());
                                        let _ = tx.send(proxy);
                                    },
                                    Err(err) => {
                                        error!("load load balance extension error: load extension failed, url: {}, err: {}", url, err);
                                    },
                                }
                            },
                            Some(load_balance_extension) => {
                                let _ = tx.send(load_balance_extension.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    async fn load_router_extension(&mut self, url: Url, tx: oneshot::Sender<router_extension::proxy::RouterProxy>) {
        debug!("load router extension, url: {}", url);
        let extension = url.query::<Extension>();
        match extension {
            None => {
                error!("load router extension error: extension not found, url: {}", url);
            },
            Some(extension) => {
                let name = extension.value();
                match self.router_extension_loaders.get_mut(&name) {
                    None => {
                        error!("load router extension error: extension loader not found, url: {}", url);
                    },
                    Some(extension_loader) => {
                        let extension_cache_key = url.to_string();
                        match self.router_extensions.get(&extension_cache_key) {
                            None => {
                                match extension_loader.load(&url).await {
                                    Ok(mut router_extension) => {
                                        let (extension_opt_tx, mut extension_opt_rx) = tokio::sync::mpsc::channel(64);
                                        tokio::spawn(async move {
                                            while let Some(opt) = extension_opt_rx.recv().await {
                                                match opt {
                                                    router_extension::proxy::RouterOpt::Route(invokers, callback) => {
                                                        let route = router_extension.route(invokers);
                                                        let _ = callback.send(route.await);
                                                    },
                                                }
                                            }
                                            debug!("router extension destroy, url: {}", url);
                                        });

                                        let proxy = router_extension::proxy::RouterProxy::new(extension_opt_tx);
                                        self.router_extensions.insert(extension_cache_key, proxy.clone());
                                        let _ = tx.send(proxy);
                                    },
                                    Err(err) => {
                                        error!("load router extension error: load extension failed, url: {}, err: {}", url, err);
                                    },
                                }
                            },
                            Some(router_extension) => {
                                let _ = tx.send(router_extension.clone());
                            }
                        }
                    }
                }
            }
        }
    }

}


struct ExtensionDirectoryCommander {
    sender: tokio::sync::mpsc::Sender<ExtensionOpt>,
}

impl ExtensionDirectoryCommander {

    fn new(sender: tokio::sync::mpsc::Sender<ExtensionOpt>) -> Self {
        ExtensionDirectoryCommander {
            sender,
        }
    }

    async fn add_protocol_extension_loader(&self, loader: Box<dyn ProtocolExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddProtocolExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add protocol extension loader failed").into()),
        }
    }

    async fn add_registry_extension_loader(&self, loader: Box<dyn RegistryExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRegistryExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add registry extension loader failed").into()),
        }
    }

    async fn add_cluster_extension_loader(&self, loader: Box<dyn ClusterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddClusterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add cluster extension loader failed").into()),
        }
    }

    async fn add_load_balance_extension_loader(&self, loader: Box<dyn LoadBalanceExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddLoadBalanceExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add load balance extension loader failed").into()),
        }
    }

    async fn add_router_extension_loader(&self, loader: Box<dyn RouterExtensionLoader + Send>) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::AddRouterExtensionLoader(loader)).await {
            Ok(_) => Ok(()),
            Err(_) => Err(AddExtensionLoaderError::new("add router extension loader failed").into()),
        }
    }

    async fn remove_protocol_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveProtocolExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove protocol extension loader failed").into()),
        }
    }

    async fn remove_registry_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRegistryExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove registry extension loader failed").into()),
        }
    }

    async fn remove_cluster_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveClusterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove cluster extension loader failed").into()),
        }
    }

    async fn remove_load_balance_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveLoadBalanceExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove load balance extension loader failed").into()),
        }
    }

    async fn remove_router_extension_loader(&self, name: &str) -> Result<(), StdError> {
        match self.sender.send(ExtensionOpt::RemoveRouterExtensionLoader(name.to_string())).await {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveExtensionLoaderError::new("remove router extension loader failed").into()),
        }
    }

    async fn load_protocol_extension(&self, url: Url) -> Result<protocol_extension::proxy::ProtocolProxy, StdError> {
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
    
    async fn load_registry_extension(&self, url: Url) -> Result<registry_extension::proxy::RegistryProxy, StdError> {
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

    async fn load_cluster_extension(&self, url: Url) -> Result<cluster_extension::proxy::ClusterProxy, StdError> {
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

    async fn load_load_balance_extension(&self, url: Url) -> Result<load_balance_extension::proxy::LoadBalanceProxy, StdError> {
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

    async fn load_router_extension(&self, url: Url) -> Result<router_extension::proxy::RouterProxy, StdError> {
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



enum ExtensionOpt {
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

    LoadProtocolExtension(Url, oneshot::Sender<protocol_extension::proxy::ProtocolProxy>),
    LoadRegistryExtension(Url, oneshot::Sender<registry_extension::proxy::RegistryProxy>),
    LoadClusterExtension(Url,  oneshot::Sender<cluster_extension::proxy::ClusterProxy>),
    LoadLoadBalanceExtension(Url, oneshot::Sender<load_balance_extension::proxy::LoadBalanceProxy>),
    LoadRouterExtension(Url,  oneshot::Sender<router_extension::proxy::RouterProxy>),
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