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
            interface_name,
            method_names,
        } = T::service_metadata();

        url.set_protocol("consumer");
        url.add_query_param(InterfaceName::new(interface_name));
        url.add_query_param(RustTypeName::new(std::any::type_name::<T>().to_string()));
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


pub struct RustTypeName(String);

impl RustTypeName {

    pub fn new(rust_type_name: String) -> Self {
        Self(rust_type_name)
    }
}

impl UrlParam for RustTypeName {
    type TargetType = String;

    fn name() -> &'static str {
        "rust-type-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for RustTypeName {
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


pub struct InvokerDirectoryExtension(String);

impl InvokerDirectoryExtension {

    pub fn new(invoker_directory_extension: String) -> Self {
        Self(invoker_directory_extension)
    }
}

impl UrlParam for InvokerDirectoryExtension {
    type TargetType = String;

    fn name() -> &'static str {
        "invoker-directory-extension"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokerDirectoryExtension {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

pub struct ClusterExtension(String);

impl ClusterExtension {

    pub fn new(cluster_extension: String) -> Self {
        Self(cluster_extension)
    }
}


impl UrlParam for ClusterExtension {
    type TargetType = String;

    fn name() -> &'static str {
        "cluster-extension"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}


impl FromStr for ClusterExtension {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}


pub struct RouterExtension(String);

impl RouterExtension {

    pub fn new(router_extension: String) -> Self {
        Self(router_extension)
    }
}

impl UrlParam for RouterExtension {
    type TargetType = String;

    fn name() -> &'static str {
        "router-extension"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for RouterExtension {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}


pub struct LoadBalanceExtension(String);

impl LoadBalanceExtension {

    pub fn new(load_balance_extension: String) -> Self {
        Self(load_balance_extension)
    }
}

impl UrlParam for LoadBalanceExtension {
    type TargetType = String;

    fn name() -> &'static str {
        "load-balance-extension"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for LoadBalanceExtension {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}