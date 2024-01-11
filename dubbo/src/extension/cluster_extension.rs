use async_trait::async_trait;

use crate::{url::Url, inv::Invoker, StdError};


#[async_trait]
pub trait Cluster {

    async fn join(&mut self, url: Url, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError>;
   
}


pub mod proxy {

    use async_trait::async_trait;
    use thiserror::Error;
    use tokio::sync::oneshot;
    use tracing::error;
    use crate::inv::Invoker;
    use crate::url::Url;
    use crate::StdError;

    use super::Cluster;

    pub enum ClusterOpt {
        Join(Url, Vec<Box<dyn Invoker + Send>>, oneshot::Sender<Result<Box<dyn Invoker + Send>, StdError>>),
    }

    pub struct ClusterProxy {
        sender: tokio::sync::mpsc::Sender<ClusterOpt>,
    }


    #[async_trait]
    impl Cluster for ClusterProxy {

        async fn join(&mut self, url: Url, invokers: Vec<Box<dyn Invoker + Send>>) -> Result<Box<dyn Invoker + Send>, StdError> {

            let (tx, rx) = oneshot::channel();

            match self.sender.send(ClusterOpt::Join(url.clone(), invokers, tx)).await {
                Ok(_) => {
                    match rx.await {
                        Ok(result) => result,
                        Err(_) => {
                            error!("cluster proxy error: receive join response failed, url: {}", url);
                            return Err(ClusterProxyError::new("receive join response failed").into());
                        },
                    }
                },
                Err(_) => {
                    error!("cluster proxy error: send join request failed, url: {}", url);
                    return Err(ClusterProxyError::new("send join request failed").into());
                },
            }
        }
    }

    #[derive(Error, Debug)]
    #[error("{0}")]
    pub struct ClusterProxyError(String);

    impl ClusterProxyError {

        pub fn new(msg: &str) -> Self {
            ClusterProxyError(msg.to_string())
        }
    }
}