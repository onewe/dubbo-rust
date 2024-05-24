use crate::params::cluster_params::ClusterType;



#[derive(Clone)]
pub struct DubboConfig;


impl DubboConfig {

    pub fn cluster(&self) -> ClusterType {
        ClusterType::new("failover")
    }
}