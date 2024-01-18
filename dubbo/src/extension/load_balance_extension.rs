use async_trait::async_trait;

use crate::{inv::Invoker, StdError};


#[async_trait]
pub trait LoadBalance {

    async fn select(&self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError>;
    
}

pub mod proxy {

    use async_trait::async_trait;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use tracing::error;
    use crate::inv::Invoker;
    use crate::StdError;

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

        async fn select(&self, invokes: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError> {
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

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub struct LoadBalanceProxyError(String);

    impl LoadBalanceProxyError {
        pub fn new(msg: &str) -> Self {
            LoadBalanceProxyError(msg.to_string())
        }
    }
}