use async_trait::async_trait;
use futures::Stream;

use crate::{url::Url, StdError};

#[async_trait]
pub trait Registry {

    async fn register(&mut self, url: Url) -> Result<(), StdError>;

    async fn unregister(&mut self, url: Url) -> Result<(), StdError>;

    async fn subscribe(&mut self, url: Url) -> Result<Box<dyn Stream<Item = Vec<String>> + Send>, StdError>;

}


pub mod proxy {

    use async_trait::async_trait;
    use futures::Stream;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use tracing::error;
    use crate::url::Url;
    use crate::StdError;

    use super::Registry;


    pub(crate) enum RegistryOpt {
        Register(Url, oneshot::Sender<Result<(), StdError>>),
        Unregister(Url, oneshot::Sender<Result<(), StdError>>),
        Subscribe(Url, oneshot::Sender<Result<Box<dyn Stream<Item = Vec<String>> + Send>, StdError>>),
        
    }

    #[derive(Clone)]
    pub struct RegistryProxy {
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

        async fn subscribe(&mut self, url: Url) -> Result<Box<dyn Stream<Item = Vec<String>> + Send>, StdError> {
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

    #[derive(Error, Debug)]
    #[error("registry proxy error: {0}")]
    pub struct RegistryProxyError(String);

    impl RegistryProxyError {

        pub fn new(msg: &str) -> Self {
            RegistryProxyError(msg.to_string())
        }
    }
}