use async_trait::async_trait;
use crate::{invoker::Invoker, StdError};



#[async_trait]
pub trait LoadBalance {

    async fn select(&mut self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError>;
    
}

pub mod proxy {

    use async_trait::async_trait;
    use dubbo_logger::tracing::error;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use crate::{invoker::Invoker, StdError};

    use super::LoadBalance;

    pub(crate) enum LoadBalanceOpt {
        Select(Vec<Box<dyn Invoker + Send>>, oneshot::Sender<Result<Box<dyn Invoker + Send>, StdError>>),
    }

    #[derive(Clone)]
    pub struct LoadBalanceProxy {
        sender: tokio::sync::mpsc::Sender<LoadBalanceOpt>,
    }

    impl LoadBalanceProxy {
            
        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<LoadBalanceOpt>) -> Self {
            LoadBalanceProxy {
                sender,
            }
        }
    }

    #[async_trait]
    impl LoadBalance for LoadBalanceProxy {

        async fn select(&mut self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError> {
            let (tx, rx) = oneshot::channel();

            match self.sender.send(LoadBalanceOpt::Select(invokes, tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("load balance proxy error: receive select response failed");
                            return Err(LoadBalanceProxyError::new("receive select response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("load balance proxy error: send select request failed");
                    return Err(LoadBalanceProxyError::new("send select request failed").into());
                },
            }
        }
    }


    impl From<Box<dyn LoadBalance + Send>> for LoadBalanceProxy {

        fn from(mut lb: Box<dyn LoadBalance + Send>) -> Self {
            let (tx, mut rx) = tokio::sync::mpsc::channel(1024);

            tokio::spawn(async move {
                while let Some(opt) = rx.recv().await {
                    match opt {
                        LoadBalanceOpt::Select(invokes, tx) => {
                            let result = lb.select(invokes).await;
                            if let Err(_) = tx.send(result) {
                                error!("load balance proxy error: send select response failed");
                            }
                        },
                    }
                }
            });

            LoadBalanceProxy {
                sender: tx,
            }
        }
    }

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub struct LoadBalanceProxyError(String);

    impl LoadBalanceProxyError {
        pub fn new(msg: &str) -> Self {
            LoadBalanceProxyError(msg.to_string())
        }
    }
}