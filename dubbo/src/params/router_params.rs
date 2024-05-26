use std::str::FromStr;

use crate::url::UrlParam;

pub struct RouterName(String);


impl RouterName {

    pub fn new(name: impl Into<String>) -> Self {
        RouterName(name.into())
    }
}


impl UrlParam for RouterName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "router-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for RouterName {
    
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RouterName::new(s))
    }
}


pub struct RouterServiceName(String);

impl RouterServiceName {

    pub fn new(name: impl Into<String>) -> Self {
        RouterServiceName(name.into())
    }
}


impl UrlParam for RouterServiceName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "router-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for RouterServiceName {
    
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RouterServiceName::new(s))
    }
}

#[derive(Debug, Clone)]
pub struct RouterType(String);

impl RouterType {

    pub fn new(name: impl Into<String>) -> Self {
        RouterType(name.into())
    }
}

impl UrlParam for RouterType {
    
    type TargetType = String;

    fn name() -> &'static str {
        "router-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for RouterType {
    
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RouterType::new(s))
    }
}