use async_trait::async_trait;

use crate::{invoker::Invoker, StdError};

#[async_trait]
pub trait Router {
    
    async fn route(&mut self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Vec<Box<dyn Invoker + Send>>, StdError>;
}


pub(crate) mod proxy {

    use async_trait::async_trait;
    use dubbo_logger::tracing::error;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use crate::{invoker::Invoker, StdError};

    use super::Router;

    pub(crate) enum RouterOpt {
        Route(Vec<Box<dyn Invoker + Send>>, oneshot::Sender<Result<Vec<Box<dyn Invoker + Send>>, StdError>>),
    }

    #[derive(Clone)]
    pub(crate) struct RouterProxy {
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

        async fn route(&mut self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Vec<Box<dyn Invoker + Send>>, StdError> {
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

    impl From<Box<dyn Router + Send>> for RouterProxy {

        fn from(mut router: Box<dyn Router + Send>) -> Self {
            let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);

            tokio::spawn(async move {
                while let Some(opt) = receiver.recv().await {
                    match opt {
                        RouterOpt::Route(invokes, tx) => {
                            let result = router.route(invokes).await;
                            let _ = tx.send(result);
                        },
                    }
                }
            });

            RouterProxy {
                sender,
            }
        }
    }

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub(crate) struct RouterProxyError(String);

    impl RouterProxyError {
        pub(crate) fn new(msg: &str) -> Self {
            RouterProxyError(msg.to_string())
        }
    }
}

