use std::collections::HashMap;

use crate::extension::property_source_extension::ConfigProperty;

use super::Configuration;


#[derive(Debug, Default, Clone)]
pub struct ClusterConfig {
    proeprties: HashMap<String, String>,
}

impl ClusterConfig {

    pub fn cluster_type(mut self, val: impl Into<String>) -> Self{
        let cluster_type = ClusterType::create(val.into());
        self.proeprties.extend(cluster_type.into_map());
        self
    }
}


impl Configuration for ClusterConfig {


    fn property(&self, key: &str) -> Option<&String> {
        self.proeprties.get(key)
    }

    fn properties(&self) -> HashMap<String, String> {
        self.proeprties.clone()
    }
}




pub struct ClusterType(String);

impl ClusterType {
    const KEY: &'static str = "dubbo.cluster.type";
}

impl ConfigProperty for ClusterType {
    
    type Target = String;

    fn create(value: Self::Target) -> Self {
        ClusterType(value)
    }

    fn create_from_map(map: &HashMap<String, String>) -> Option<Self> {
        map.get(Self::KEY).map(|val|ClusterType(val.clone()))
    }

    fn value(&self) -> Self::Target {
        self.0.clone()
    }

    fn into_map(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(Self::KEY.to_owned(), self.0);
        map
    }
}