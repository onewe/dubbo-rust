use std::sync::Arc;
use async_trait::async_trait;
use crate::extension::invoker_extension::Invoker;
use crate::Url;

// url: route://0.0.0.0?name=route&service-name=hello&type=tag-router
// extension_url: extension://0.0.0.0?extension-type=route&extension-name=tag-router&route-url=route://0.0.0.0?name=route&service-name=hello&type=tag-router
#[async_trait]
pub trait Router {
    async fn route(&self, invokers: Vec<Arc<dyn Invoker>>) -> Arc<dyn Invoker>;
    
    fn url(&self) -> &Url;
}