use std::collections::HashSet;

use async_trait::async_trait;
use dubbo_base::Url;
use tokio::sync::watch;

use crate::StdError;


#[async_trait]
pub trait Registry {

    async fn register(&mut self, url: Url) -> Result<(), StdError>;

    async fn unregister(&mut self, url: Url) -> Result<(), StdError>;

    async fn subscribe(&mut self, url: Url) -> Result<watch::Receiver<HashSet<String>>, StdError>;

}


pub(crate) mod proxy {

    use std::collections::HashSet;

    use async_trait::async_trait;
    use dubbo_base::Url;
    use tracing::error;
    use thiserror::Error;
    use tokio::sync::{oneshot, watch};
    use crate::StdError;

    use super::Registry;


    pub(crate) enum RegistryOpt {
        Register(Url, oneshot::Sender<Result<(), StdError>>),
        Unregister(Url, oneshot::Sender<Result<(), StdError>>),
        Subscribe(Url, oneshot::Sender<Result<watch::Receiver<HashSet<String>>, StdError>>),
        
    }

    #[derive(Clone)]
    pub(crate) struct RegistryProxy {
        sender: tokio::sync::mpsc::Sender<RegistryOpt>,
    }

    impl RegistryProxy {
        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<RegistryOpt>) -> Self {
            RegistryProxy {
                sender,
            }
        }
    }

    #[async_trait]
    impl Registry for RegistryProxy {

        async fn register(&mut self, url: Url) -> Result<(), StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(RegistryOpt::Register(url.clone(), tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("registry proxy error: receive register response failed, url: {}", url);
                            return Err(RegistryProxyError::new("receive register response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("registry proxy error: send register request failed, url: {}", url);
                    return Err(RegistryProxyError::new("send register opt failed").into());
                },
            }
        }

        async fn unregister(&mut self, url: Url) -> Result<(), StdError> {
            let (tx, rx) = oneshot::channel();
            match self.sender.send(RegistryOpt::Unregister(url.clone(), tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("registry proxy error: receive unregister response failed, url: {}", url);
                            return Err(RegistryProxyError::new("receive unregister response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("registry proxy error: send unregister request failed, url: {}", url);
                    return Err(RegistryProxyError::new("send unregister opt failed").into());
                },
            }
        }

        async fn subscribe(&mut self, url: Url) -> Result<watch::Receiver<HashSet<String>>, StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(RegistryOpt::Subscribe(url.clone(), tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("registry proxy error: receive subscribe response failed, url: {}", url);
                            return Err(RegistryProxyError::new("receive subscribe response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("registry proxy error: send subscribe request failed, url: {}", url);
                    return Err(RegistryProxyError::new("send subscribe opt failed").into());
                },
            }
        }
    }

    impl From<Box<dyn Registry + Send>> for RegistryProxy {
            
            fn from(mut registry: Box<dyn Registry + Send>) -> Self {
    
                let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);
    
                tokio::spawn(async move {
                    while let Some(opt) = receiver.recv().await {
                        match opt {
                            RegistryOpt::Register(url, tx) => {
                                let register = registry.register(url).await;
                                if let Err(_) = tx.send(register) {
                                    error!("registry proxy error: send register response failed");
                                }
                            },
                            RegistryOpt::Unregister(url, tx) => {
                                let unregister = registry.unregister(url).await;
                                if let Err(_) = tx.send(unregister) {
                                    error!("registry proxy error: send unregister response failed");
                                }
                            },
                            RegistryOpt::Subscribe(url, tx) => {
                                let subscribe = registry.subscribe(url).await;
                                if let Err(_) = tx.send(subscribe) {
                                    error!("registry proxy error: send subscribe response failed");
                                }
                            },
                        }
                    }
                });
    
                RegistryProxy::new(sender)
            }
    }

    #[derive(Error, Debug)]
    #[error("registry proxy error: {0}")]
    pub(crate) struct RegistryProxyError(String);

    impl RegistryProxyError {

        pub(crate) fn new(msg: &str) -> Self {
            RegistryProxyError(msg.to_string())
        }
    }
}