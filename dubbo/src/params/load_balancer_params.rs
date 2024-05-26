use std::str::FromStr;

use crate::{url::UrlParam, StdError};

pub struct LoadBalancerName(String);


impl LoadBalancerName {

    pub fn new(name: impl Into<String>) -> Self {
        LoadBalancerName(name.into())
    }
}


impl UrlParam for LoadBalancerName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "load-balancer-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for LoadBalancerName {
    
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LoadBalancerName(s.to_string()))
    }
}


pub struct LoadBalancerServiceName(String);

impl LoadBalancerServiceName {
    
    pub fn new(name: impl Into<String>) -> Self {
        LoadBalancerServiceName(name.into())
    }
}

impl UrlParam for LoadBalancerServiceName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "load-balancer-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for LoadBalancerServiceName {
    
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LoadBalancerServiceName(s.to_string()))
    }
}


#[derive(Debug, Clone)]
pub struct LoadBalancerType(String);


impl LoadBalancerType {
    
    pub fn new(name: impl Into<String>) -> Self {
        LoadBalancerType(name.into())
    }
}

impl UrlParam for LoadBalancerType {
    
    type TargetType = String;

    fn name() -> &'static str {
        "load-balancer-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for LoadBalancerType {
    
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LoadBalancerType(s.to_string()))
    }
}