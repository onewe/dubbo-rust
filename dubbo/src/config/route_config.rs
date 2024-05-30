
#[derive(Debug, Default, Clone)]
pub struct RouteConfig {
    r_type: String,
}

impl RouteConfig {
    pub fn new(r_type: String) -> Self {
        RouteConfig {
            r_type,
        }
    }
}

impl RouteConfig {
    pub fn route_type(&self) -> &str {
        &self.r_type
    }

    pub fn route_type_mut(&mut self) -> &mut String {
        &mut self.r_type
    }
}