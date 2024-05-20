use std::sync::Arc;
use async_trait::async_trait;
use crate::extension::invoker_directory_extension::InvokerList;
use crate::extension::invoker_extension::Invoker;
use crate::extension::route_extension::Router;
use crate::{StdError, Url};

// url: load-balancer://0.0.0.0?name=random&service-name=hello&type=Random
// extension_url: extension://0.0.0.0?extension-type=load-balancer&extension-name=random-load-balancer&load-balancer-url=load-balancer://0.0.0.0?name=random&service-name=hello&type=random-load-balancer
#[async_trait]
pub trait LoadBalancer {
    async fn load_balancer(&mut self, invoker_list: Box<dyn InvokerList + Send + 'static>, router: Box<dyn Router + Send + 'static>) -> Box<dyn LoadBalancerChooser + Send + 'static>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn LoadBalancer + Send + 'static>;
}



// url: load-balancer-chooser://0.0.0.0?name=random-chooser&service-name=hello&type=random-load-balancer-chooser
#[async_trait]
pub trait LoadBalancerChooser {
    async fn choose(&mut self) -> Box<dyn Invoker + Send + 'static>;

    async fn ready(&mut self) -> Result<(), StdError>;
    
    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn LoadBalancerChooser + Send + 'static>;
}
