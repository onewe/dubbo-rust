use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use crate::extension::invoker_extension::Invoker;
use crate::extension::registry_extension::Registry;
use crate::Url;


// url: invoker-directory://0.0.0.0?service-name=hello&type=invoker-directory
// extension_url: extension://0.0.0.0?extension-type=invoker-directory&extension-name=invoker-directory&invoker-directory-url=invoker-directory://0.0.0.0?service-name=hello&type=invoker-directory
type Converter = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Invoker + Send + 'static>, crate::StdError>>>>;
#[async_trait]
pub trait InvokerDirectory {
    
    async fn directory(&mut self, service_subscribe: Url, converter: Converter, registry: Box<dyn Registry + Send + 'static>) -> Box<dyn InvokerList + Send + 'static>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn InvokerDirectory + Send + 'static>;
    
}


// url: invoker-list://0.0.0.0?service-name=hello&type=invoker-list
#[async_trait]
pub trait InvokerList {
    
    async fn list(&self) -> Vec<Arc<dyn Invoker>>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn InvokerList + Send + 'static>;
    
}