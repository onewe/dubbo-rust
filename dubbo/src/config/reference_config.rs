use std::str::FromStr;

use dubbo_base::{url::UrlParam, Url};

use crate::{framework::{DubboService, ServiceMetadata}, StdError};

pub struct ReferenceConfig {
    inner: Url,
}



impl ReferenceConfig {


    pub fn new<T>() -> Self 
    where
        T: DubboService + 'static,
    {
        
        let mut url = Url::empty();
        let ServiceMetadata {
            service_name,
            method_names,
        } = T::service_metadata();

        url.set_protocol("reference");
        url.add_query_param(InterfaceName::new(service_name));
        url.add_query_param(InterfaceTypeId::new(T::type_id()));
        url.add_query_param(MethodNames::new(method_names));
     
        
        Self {
            inner: url,
        }
    }

    pub fn url(&self) -> &Url {
        &self.inner
    }

}



pub struct InterfaceName(String);

impl InterfaceName {

    pub fn new(interface_name: String) -> Self {
        Self(interface_name)
    }
}

impl UrlParam for InterfaceName {
    type TargetType = String;

    fn name() -> &'static str {
        "interface"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for InterfaceName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

pub struct InterfaceTypeId(String);

impl InterfaceTypeId {

    pub fn new(interface_type_id: String) -> Self {
        Self(interface_type_id)
    }
}


impl UrlParam for InterfaceTypeId {
    type TargetType = String;

    fn name() -> &'static str {
        "interface_type_id"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for InterfaceTypeId {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}


pub struct MethodNames(Vec<String>);

impl MethodNames {

    pub fn new(method_names: Vec<String>) -> Self {
        Self(method_names)
    }
}

impl UrlParam for MethodNames {
    type TargetType = Vec<String>;

    fn name() -> &'static str {
        "methods"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.join(",").into()
    }
}

impl FromStr for MethodNames {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(",").map(|s| s.to_string()).collect()))
    }
}
