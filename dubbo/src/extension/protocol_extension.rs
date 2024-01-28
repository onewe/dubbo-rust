use async_trait::async_trait;
use dubbo_base::Url;

use crate::{invoker::Invoker, StdError};


#[async_trait]
pub trait Protocol {
    
    async fn export(&mut self, url: Url) -> Result<(), StdError>;

    async fn refer(&mut self, url: Url) -> Result<Box<dyn Invoker + Send>, StdError>;
}

pub(crate) mod proxy {

    use async_trait::async_trait;
    use dubbo_base::Url;
    use tracing::error;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use crate::{invoker::Invoker, StdError};

    use super::Protocol;


    pub(crate) enum ProtocolOpt {
        Export(Url, oneshot::Sender<Result<(), StdError>>),
        Refer(Url, oneshot::Sender<Result<Box<dyn Invoker + Send>, StdError>>),
    }

    #[derive(Clone)]
    pub(crate) struct ProtocolProxy {
        sender: tokio::sync::mpsc::Sender<ProtocolOpt>,
    }

    impl ProtocolProxy {

        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<ProtocolOpt>) -> Self {
            ProtocolProxy {
                sender,
            }
        }
    }


    #[async_trait]
    impl Protocol for ProtocolProxy {

        async fn export(&mut self, url: Url) -> Result<(), StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(ProtocolOpt::Export(url.clone(), tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("protocol proxy error: receive export response failed, url: {}", url);
                            return Err(ProtocolProxyError::new("receive export response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("protocol proxy error: send export request failed, url: {}", url);
                    return Err(ProtocolProxyError::new("send export request failed").into());
                },
            }
        }

        async fn refer(&mut self, url: Url) -> Result<Box<dyn Invoker + Send>, StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(ProtocolOpt::Refer(url.clone(), tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("protocol proxy error: receive refer response failed, url: {}", url);
                            return Err(ProtocolProxyError::new("receive refer response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("protocol proxy error: send refer request failed, url: {}", url);
                    return Err(ProtocolProxyError::new("send refer request failed").into());
                },
            }
        }
    }


    impl From<Box<dyn Protocol + Send>> for ProtocolProxy {
            
            fn from(mut protocol: Box<dyn Protocol + Send>) -> Self {
                let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);
    
                tokio::spawn(async move {
                    while let Some(opt) = receiver.recv().await {
                        match opt {
                            ProtocolOpt::Export(url, tx) => {
                                let export = protocol.export(url).await;
                                if let Err(_) = tx.send(export) {
                                        error!("protocol proxy error: send export response failed");
                                }
                            },
                            ProtocolOpt::Refer(url, tx) => {
                                let refer = protocol.refer(url).await;
                                if let Err(_) = tx.send(refer) {
                                    error!("protocol proxy error: send refer response failed");
                                }
                            },
                        }
                    }
                });
    
                ProtocolProxy::new(sender)
            }
    }

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub(crate) struct ProtocolProxyError(String);

    impl ProtocolProxyError {

        pub(crate) fn new(msg: &str) -> Self {
            ProtocolProxyError(msg.to_string())
        }
    }

}