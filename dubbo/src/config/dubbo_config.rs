use crate::common::url::{params::{cluster_params::ClusterType, invoker_direcotry_params::InvokerDirectoryType, load_balancer_params::LoadBalancerType, router_params::RouterType}, Url};


#[derive(Clone)]
pub struct DubboConfig;


impl DubboConfig {

    pub fn cluster(&self) -> ClusterType {
        ClusterType::new("failover")
    }


    pub fn load_balancer(&self) -> LoadBalancerType {
        LoadBalancerType::new("random-load-balancer")
    }


    pub fn router(&self) -> RouterType {
        RouterType::new("tag-router")
    }

    pub fn invoker_directory(&self) -> InvokerDirectoryType {
        InvokerDirectoryType::new("default-invoker-directory")
    }


    pub fn registry(&self) -> Url {
        "registry://127.0.0.1:8848?registry-type=nacos".parse().unwrap()
    }
}