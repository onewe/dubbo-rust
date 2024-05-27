use std::str::FromStr;

use crate::{common::url::UrlParam, StdError};

pub struct InvokerName(String);

impl InvokerName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl UrlParam for InvokerName {

    type TargetType = String;

    fn name() -> &'static str {
        "invoker-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for InvokerName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerName::new(s))
    }
}



pub struct InvokerProtocol(String);

impl InvokerProtocol {
    pub fn new(protocol: impl Into<String>) -> Self {
        Self(protocol.into())
    }
}

impl UrlParam for InvokerProtocol {

    type TargetType = String;

    fn name() -> &'static str {
        "invoker-protocol"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokerProtocol {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerProtocol::new(s))
    }
}


pub struct InvokerServiceName(String);

impl InvokerServiceName {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self(service_name.into())
    }
}

impl UrlParam for InvokerServiceName {

    type TargetType = String;

    fn name() -> &'static str {
        "invoker-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokerServiceName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerServiceName::new(s))
    }
}