use std::collections::HashMap;

use crate::extension::property_source_extension::ConfigProperty;

use super::Configuration;


#[derive(Debug, Default, Clone)]
pub struct InvokerDirectoryConfig {
    properties: HashMap<String, String>,
}

impl InvokerDirectoryConfig {

    pub fn invoker_directory_type(mut self, val: impl Into<String>) -> Self {
        let invoker_directory_type = InvokerDirectoryType::create(val.into());
        self.properties.extend(invoker_directory_type.into_map());
        self
    }
}


impl Configuration for InvokerDirectoryConfig {
    
    fn property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    fn properties(&self) -> HashMap<String, String> {
        self.properties.clone()
    }
}

pub struct InvokerDirectoryType(String);

impl InvokerDirectoryType {

    const KEY: &'static str = "dubbo.invoker.directory.type";
}

impl ConfigProperty for InvokerDirectoryType {
    
    type Target = String;

    fn create(value: Self::Target) -> Self {
        Self(value)
    }

    fn create_from_map(map: &HashMap<String, String>) -> Option<Self> where Self: Sized {
        map.get(Self::KEY).map(|val|InvokerDirectoryType(val.clone()))
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