use crate::cluster::support::cluster_invoker::ClusterInvoker;
use crate::registry::BoxRegistry;
use std::sync::Arc;

pub trait ClusterRegistryIntegration {
    /// get cluster invoker struct
    fn get_invoker(registry: BoxRegistry) -> Option<Arc<ClusterInvoker>>;
}