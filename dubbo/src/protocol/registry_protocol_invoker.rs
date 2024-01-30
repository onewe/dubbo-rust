use dubbo_base::Url;

use crate::{extension::{cluster_extension::Cluster, directory_extension::InvokerDirectory}, invoker::{Invoker, RpcInvocation, RpcResponse}, StdError};

pub struct RegistryProtocolInvoker {
    url: Url,
    directory: Box<dyn InvokerDirectory + Send>,
    cluster: Box<dyn Cluster + Send>,
}


impl RegistryProtocolInvoker {

    pub fn new(url: Url, directory: Box<dyn InvokerDirectory + Send>, cluster: Box<dyn Cluster + Send>) -> Self {
        Self {
            url,
            directory,
            cluster,
        }
    }
}

#[async_trait::async_trait]
impl Invoker for RegistryProtocolInvoker {

    async fn poll_ready(&mut self) -> Result<(), StdError> {
        Ok(())
    }

    async fn invoke(&mut self, invocation: RpcInvocation) -> Result<RpcResponse, StdError> {

        let invokers = self.directory.list().await?;

        // route

        let route_url = self.url.clone();

        // load balance        


        todo!()
    }

    fn url(&self) -> &Url {
        &self.url
    }
  
}