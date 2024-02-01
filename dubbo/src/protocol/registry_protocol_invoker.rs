use dubbo_base::Url;
use thiserror::Error;

use crate::{extension::{cluster_extension::Cluster, directory_extension::InvokerDirectory}, invoker::{Invoker, RpcInvocation, RpcResponse}, StdError};

pub struct RegistryProtocolInvoker {
    url: Url,
    directory: Box<dyn InvokerDirectory + Send>,
    cluster: Box<dyn Cluster + Send>,
    inner_invoker: Option<Box<dyn Invoker + Send>>,
}


impl RegistryProtocolInvoker {

    pub const NAME: &'static str = "registry-invoker";

    pub fn new(url: Url, directory: Box<dyn InvokerDirectory + Send>, cluster: Box<dyn Cluster + Send>,) -> Self {
        Self {
            url,
            directory,
            cluster,
            inner_invoker: None,
        }
    }
}

#[async_trait::async_trait]
impl Invoker for RegistryProtocolInvoker {

    async fn ready(&mut self) -> Result<(), StdError> {
        
        let invokers = self.directory.list().await?;

        let invoker = self.cluster.join(invokers).await?;

        self.inner_invoker = Some(invoker);

        Ok(())
    }

    async fn invoke(&mut self, invocation: RpcInvocation) -> Result<RpcResponse, StdError> {

       match self.inner_invoker.as_mut() {
           Some(invoker) => invoker.invoke(invocation).await,
           None => Err(NoAvailableInvokerError::new("inner invoker is none").into()),
       }
    }

    fn url(&self) -> &Url {
        &self.url
    }
  
}


#[derive(Error, Debug)]
#[error("registry protocol invoker error: {0}")]
pub struct NoAvailableInvokerError(String);

impl NoAvailableInvokerError {

    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
    
}
