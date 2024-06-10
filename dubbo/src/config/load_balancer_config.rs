use std::collections::HashMap;

use crate::extension::property_source_extension::ConfigProperty;

use super::Configuration;

#[derive(Debug, Default, Clone)]
pub struct LoadBalancerConfig {
    properties: HashMap<String, String>,
}

impl LoadBalancerConfig {

    pub fn load_balancer_type(mut self, val: impl Into<String>) -> Self {
        let load_balancer_type = LoadBalancerType::create(val.into());
        self.properties.extend(load_balancer_type.into_map());
        self
    }
}


impl Configuration for LoadBalancerConfig {
    fn property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    fn properties(&self) -> HashMap<String, String> {
        self.properties.clone()
    }
}


pub struct LoadBalancerType(String);

impl LoadBalancerType {
    const KEY: &'static str = "dubbo.loadbalancer.type";
}


impl ConfigProperty for LoadBalancerType {
    type Target = String;

    fn create(value: Self::Target) -> Self {
        LoadBalancerType(value)
    }

    fn create_from_map(map: &HashMap<String, String>) -> Option<Self> {
        map.get(Self::KEY).map(|val|LoadBalancerType(val.clone()))
    }

    fn value(&self) -> Self::Target {
        self.0.clone()
    }

    fn into_map(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(Self::KEY.to_string(), self.0);
        map
    }
}

