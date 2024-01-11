use async_trait::async_trait;

use crate::{url::Url, StdError, inv::Invoker};


#[async_trait]
pub trait Protocol {
    
    async fn export(&mut self, url: Url) -> Result<(), StdError>;

    async fn refer(&mut self, url: Url) -> Result<Box<dyn Invoker + Send>, StdError>;
}

pub mod proxy {

    use async_trait::async_trait;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use tracing::error;
    use crate::inv::Invoker;
    use crate::url::Url;
    use crate::StdError;

    use super::Protocol;


    pub enum ProtocolOpt {
        Export(Url, oneshot::Sender<Result<(), StdError>>),
        Refer(Url, oneshot::Sender<Result<Box<dyn Invoker + Send>, StdError>>),
    }

    pub struct ProtocolProxy {
        sender: tokio::sync::mpsc::Sender<ProtocolOpt>,
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

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub struct ProtocolProxyError(String);

    impl ProtocolProxyError {

        pub fn new(msg: &str) -> Self {
            ProtocolProxyError(msg.to_string())
        }
    }

}