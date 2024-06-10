

use std::collections::HashMap;

use crate::{common::url::Url, extension::property_source_extension::ConfigProperty};

#[derive(Debug, Clone, Default)]
pub struct PropertySourceConfig {
    properties: HashMap<String, String>,

}




pub struct PropertySourceUrls(Vec<Url>);


impl PropertySourceUrls {

    const KEY: &'static str = "dubbo.config.sources.urls";
}


impl ConfigProperty for PropertySourceUrls {
    
    type Target = Vec<Url>;

    fn create(value: Self::Target) -> Self {
        PropertySourceUrls(value)
    }

    fn create_from_map(map: &HashMap<String, String>) -> Option<Self> where Self: Sized {
        todo!()
    }

    fn value(&self) -> Self::Target {
        todo!()
    }

    fn into_map(self) -> HashMap<String, String> {
        todo!()
    }
}