use std::sync::Arc;
use async_trait::async_trait;
use crate::extension::invoker_directory_extension::InvokerList;
use crate::extension::invoker_extension::Invoker;
use crate::extension::route_extension::Router;
use crate::Url;

// url: load-balancer://0.0.0.0?name=random&service-name=hello&type=Random
// extension_url: extension://0.0.0.0?extension-type=load-balancer&extension-name=random-load-balancer&load-balancer-url=load-balancer://0.0.0.0?name=random&service-name=hello&type=random-load-balancer
#[async_trait]
pub trait LoadBalancer {
    async fn load_balancer(&self, invoker_list: Arc<dyn InvokerList>, router: Arc<dyn Router>) -> Box<dyn LoadBalancerChooser>;

    fn url(&self) -> &Url;
}



// url: load-balancer-chooser://0.0.0.0?name=random-chooser&service-name=hello&type=random-load-balancer-chooser
#[async_trait]
pub trait LoadBalancerChooser {
    async fn choose(&self) -> Arc<dyn Invoker>;
    
    fn url(&self) -> &Url;
}
