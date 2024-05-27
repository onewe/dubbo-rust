use std::str::FromStr;

use crate::{common::url::UrlParam, StdError};

pub struct ProtocolName(String);


impl ProtocolName {

    pub fn new(name: impl Into<String>) -> Self {
        ProtocolName(name.into())
    }
}


impl UrlParam for ProtocolName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "protocol-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for ProtocolName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProtocolName::new(s))
    }
}


pub struct ProtocolType(String);


impl ProtocolType {

    pub fn new(name: impl Into<String>) -> Self {
        ProtocolType(name.into())
    }
}

impl UrlParam for ProtocolType {
    
    type TargetType = String;

    fn name() -> &'static str {
        "protocol-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for ProtocolType {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProtocolType::new(s))
    }
}