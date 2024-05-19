use std::sync::Arc;

use async_trait::async_trait;

use crate::extension::invoker_extension::Invoker;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::Url;

// url: cluster://0.0.0.0?name=failover_cluster&service_name=hello&type=failover
// extension_url: extension://0.0.0.1?extension-type=cluster&extension-name=failover-cluster&cluster-url=cluster://0.0.0.0?name=failover_cluster&service_name=hello&type=failover
#[async_trait]
pub trait Cluster {
    
    async fn join(&self, load_balancer: Arc<dyn LoadBalancerChooser>) -> Arc<dyn Invoker>;
    
    fn url(&self) -> &Url;
}