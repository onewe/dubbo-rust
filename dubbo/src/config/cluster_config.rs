
#[derive(Debug, Default, Clone)]
pub struct ClusterConfig {
    r_type: String,
}

impl ClusterConfig {

    pub fn cluster_type(&self) -> &str {
        &self.r_type
    }

    pub fn cluster_type_mut(&mut self) -> &mut String {
        &mut self.r_type
    }
}
