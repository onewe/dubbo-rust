use std::str::FromStr;

use dubbo_base::{url::UrlParam, Url};

use crate::{config::registry_config::RegistryUrl, extension::{protocol_extension::Protocol, ProtocolExtensionLoader}, invoker::Invoker, StdError};

pub struct RegistryProtocolLoader;



#[async_trait::async_trait]
impl ProtocolExtensionLoader for RegistryProtocolLoader {
   
    fn name(&self) -> String {
        "registry".to_string()
    }

    async fn load(&mut self, url: &Url) -> Result<Box<dyn Protocol + Send>, StdError> {
        let registry_url = url.query::<RegistryUrl>().unwrap();

        let mut registry_url = registry_url.value();
        let protocol = registry_url.protocol();
        
        registry_url.add_query_param(RegistryType::new(protocol.to_string()));
        registry_url.set_protocol("registry");
        

       Ok(Box::new(RegistryProtocol::new(registry_url)))
    }
}


pub struct RegistryProtocol {
    registry_url: Url
}


impl RegistryProtocol {
    
    pub fn new(registry_url: Url) -> Self {
        Self {
            registry_url,
        }
    }

    
}

#[async_trait::async_trait]
impl Protocol for RegistryProtocol {

    async fn export(&mut self, url: Url) -> Result<(), StdError> {
        Ok(())
    }

    async fn refer(&mut self, reference_url: Url) -> Result<Box<dyn Invoker + Send>, StdError> {
        let registry_type = self.registry_url.query::<RegistryType>().unwrap();
        let registry_type = registry_type.value();
        
        
        

        todo!()
    }
   
}


pub struct RegistryType(String);

impl RegistryType {
    pub fn new(registry_type: String) -> Self {
        Self(registry_type)
    }
}

impl UrlParam for RegistryType {
    type TargetType = String;

    fn name() -> &'static str {
        "registry_type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for RegistryType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}