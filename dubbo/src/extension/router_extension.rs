use async_trait::async_trait;

use crate::{inv::Invoker, StdError};

#[async_trait]
pub trait Router {
    
    async fn route(&self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Vec<Box<dyn Invoker + Send>>, StdError>;
}


pub mod proxy {

    use async_trait::async_trait;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use tracing::error;
    use crate::inv::Invoker;
    use crate::StdError;

    use super::Router;

    pub(crate) enum RouterOpt {
        Route(Vec<Box<dyn Invoker + Send>>, oneshot::Sender<Result<Vec<Box<dyn Invoker + Send>>, StdError>>),
    }

    #[derive(Clone)]
    pub struct RouterProxy {
        sender: tokio::sync::mpsc::Sender<RouterOpt>,
    }

    impl RouterProxy {

        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<RouterOpt>) -> Self {
            RouterProxy {
                sender,
            }
        }
    }

    #[async_trait]
    impl Router for RouterProxy {

        async fn route(&self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Vec<Box<dyn Invoker + Send>>, StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(RouterOpt::Route(invokes, tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("router proxy error: receive route response failed");
                            return Err(RouterProxyError::new("receive route response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("router proxy error: send route request failed");
                    return Err(RouterProxyError::new("send route request failed").into());
                },
            }
        }
    }

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub struct RouterProxyError(String);

    impl RouterProxyError {
        pub fn new(msg: &str) -> Self {
            RouterProxyError(msg.to_string())
        }
    }
}

