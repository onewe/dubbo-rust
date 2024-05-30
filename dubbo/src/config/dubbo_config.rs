use crate::common::url::{params::{cluster_params::ClusterType, invoker_direcotry_params::InvokerDirectoryType, load_balancer_params::LoadBalancerType, router_params::RouterType}, Url};

use super::{cluster_config::ClusterConfig, invoker_directory_config::InvokerDirectoryConfig, load_balancer_config::LoadBalancerConfig, property_source_config::PropertySourceConfig, registry_config::RegistryConfig, route_config::RouteConfig};


#[derive(Clone, Default)]
pub struct DubboConfig {
    pub(crate) cluster_config: ClusterConfig,
    pub(crate) invoker_directory_config: InvokerDirectoryConfig,
    pub(crate) load_balancer_config: LoadBalancerConfig,
    pub(crate) property_source_config: PropertySourceConfig,
    pub(crate) registry_config: RegistryConfig,
    pub(crate) route_config: RouteConfig,
}


impl DubboConfig {

    
    pub fn cluster_config(self, cluster_config: ClusterConfig) -> Self {
        DubboConfig {
            cluster_config,
            ..self
        }
    }

    pub fn invoker_directory_config(self, invoker_directory_config: InvokerDirectoryConfig) -> Self {
        DubboConfig {
            invoker_directory_config,
            ..self
        }
    }

    pub fn load_balancer_config(self, load_balancer_config: LoadBalancerConfig) -> Self {
        DubboConfig {
            load_balancer_config,
            ..self
        }
    }

    pub fn property_source_config(self, property_source_config: PropertySourceConfig) -> Self {
        DubboConfig {
            property_source_config,
            ..self
        }
    }

    pub fn registry_config(self, registry_config: RegistryConfig) -> Self {
        DubboConfig {
            registry_config,
            ..self
        }
    }

    pub fn route_config(self, route_config: RouteConfig) -> Self {
        DubboConfig {
            route_config,
            ..self
        }
    }


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