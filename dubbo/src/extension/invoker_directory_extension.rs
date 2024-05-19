use std::sync::Arc;
use async_trait::async_trait;
use crate::extension::invoker_extension::Invoker;
use crate::extension::registry_extension::Registry;
use crate::Url;


// url: invoker-directory://0.0.0.0?service-name=hello&type=invoker-directory
// extension_url: extension://0.0.0.0?extension-type=invoker-directory&extension-name=invoker-directory&invoker-directory-url=invoker-directory://0.0.0.0?service-name=hello&type=invoker-directory
type Converter = fn(Url) -> Arc<dyn Invoker>;
#[async_trait]
pub trait InvokerDirectory {
    
    async fn directory(&self, service_subscribe: Url, converter: Converter, registry: Arc<dyn Registry>) -> Arc<dyn InvokerList>;

    fn url(&self) -> &Url;
    
}


// url: invoker-list://0.0.0.0?service-name=hello&type=invoker-list
#[async_trait]
pub trait InvokerList {
    
    async fn list(&self) -> Vec<Arc<dyn Invoker>>;

    fn url(&self) -> &Url;
    
}