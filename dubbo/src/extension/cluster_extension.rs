use async_trait::async_trait;
use crate::{invoker::Invoker, StdError};




#[async_trait]
pub trait Cluster {

    async fn join(&mut self, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError>;
   
}



pub(crate) mod proxy {

    use async_trait::async_trait;
    use dubbo_logger::tracing::error;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use crate::{invoker::Invoker, StdError};

    use super::Cluster;

    pub(crate) enum ClusterOpt {
        Join(Vec<Box<dyn Invoker + Send>>, oneshot::Sender<Result<Box<dyn Invoker + Send>, StdError>>),
    }

    #[derive(Clone)]
    pub(crate) struct ClusterProxy {
        sender: tokio::sync::mpsc::Sender<ClusterOpt>,
    }

    impl ClusterProxy {

        pub(crate) fn new(sender: tokio::sync::mpsc::Sender<ClusterOpt>) -> Self {
            ClusterProxy {
                sender,
            }
        }
    }


    #[async_trait]
    impl Cluster for ClusterProxy {

        async fn join(&mut self, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError> {

            let (tx, rx) = oneshot::channel();

            match self.sender.send(ClusterOpt::Join(invokers, tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("cluster proxy error: receive join response failed");
                            return Err(ClusterProxyError::new("receive join response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("cluster proxy error: send join request failed");
                    return Err(ClusterProxyError::new("send join request failed").into());
                },
            }
        }
    }

    impl From<Box<dyn Cluster + Send>> for ClusterProxy {

        fn from(mut cluster: Box<dyn Cluster + Send>) -> Self {

            let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);

            tokio::spawn(async move {
                while let Some(opt) = receiver.recv().await {
                    match opt {
                        ClusterOpt::Join(invokers, tx) => {
                            let result = cluster.join(invokers).await;
                            if let Err(_) = tx.send(result) {
                                error!("cluster proxy error: send join response failed");
                            }
                        },
                    }
                }
            });

            ClusterProxy::new(sender)
        }
    }
 
    #[derive(Error, Debug)]
    #[error("{0}")]
    pub(crate) struct ClusterProxyError(String);

    impl ClusterProxyError {

        pub(crate) fn new(msg: &str) -> Self {
            ClusterProxyError(msg.to_string())
        }
    }
}