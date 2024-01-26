use async_trait::async_trait;

use crate::{invoker::Invoker, StdError};



#[async_trait]
pub trait InvokerDirectory {
    
    async fn list(&mut self) -> Result<Vec<Box<dyn Invoker + Send>>, StdError>;
}


pub(crate) mod proxy {
    use thiserror::Error;
    use tokio::sync::oneshot;
    use dubbo_logger::tracing::error;
    use async_trait::async_trait;

    use crate::{invoker::Invoker, StdError};

    use super::InvokerDirectory;

    pub(crate) enum InvokerDirectoryOpt {
        List(oneshot::Sender<Result<Vec<Box<dyn Invoker + Send>>, StdError>>),
    }

    #[derive(Clone)]
    pub(crate) struct InvokerDirectoryProxy {
        sender: tokio::sync::mpsc::Sender<InvokerDirectoryOpt>,
    }

    impl InvokerDirectoryProxy {
        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<InvokerDirectoryOpt>) -> Self {
            InvokerDirectoryProxy {
                sender,
            }
        }
    }

    #[async_trait]
    impl InvokerDirectory for InvokerDirectoryProxy {

        async fn list(&mut self) -> Result<Vec<Box<dyn Invoker + Send>>, StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(InvokerDirectoryOpt::List(tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("invoker directory proxy error: receive list response failed");
                            return Err(InvokerDirectoryProxyError::new("receive list response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("invoker directory proxy error: send list request failed");
                    return Err(InvokerDirectoryProxyError::new("send list request failed").into());
                },
            }
        }
    }


    impl From<Box<dyn InvokerDirectory + Send>> for InvokerDirectoryProxy {

        fn from(mut invoker_directory: Box<dyn InvokerDirectory + Send>) -> Self {

            let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);

            tokio::spawn(async move {
                while let Some(opt) = receiver.recv().await {
                    match opt {
                        InvokerDirectoryOpt::List(tx) => {
                            let invokers = invoker_directory.list().await;
                            if let Err(_) = tx.send(invokers) {
                                error!("invoker directory proxy error: send list response failed");
                            }
                        },
                    }
                }
            });

            InvokerDirectoryProxy::new(sender)
        }
    }


    #[derive(Error, Debug)]
    #[error("{0}")]
    pub(crate) struct InvokerDirectoryProxyError(String);

    impl InvokerDirectoryProxyError {
        pub(crate) fn new(message: &str) -> Self {
            InvokerDirectoryProxyError(message.to_string())
        }
    }
}