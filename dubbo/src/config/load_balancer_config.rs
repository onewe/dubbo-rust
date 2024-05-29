#[derive(Debug, Clone, Default)]
pub struct LoadBalancerConfig {
    r_type: String,
}

impl LoadBalancerConfig {
    pub fn load_balancer_type(&self) -> &str {
        &self.r_type
    }

    pub fn load_balancer_type_mut(&mut self) -> &mut String {
        &mut self.r_type
    }
}