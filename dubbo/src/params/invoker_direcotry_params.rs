use std::str::FromStr;

use crate::{url::UrlParam, StdError};

pub struct InvokerDirectoryName(String);

impl InvokerDirectoryName {

    pub fn new(name: impl Into<String>) -> Self {
        InvokerDirectoryName(name.into())
    }
}


impl UrlParam for InvokerDirectoryName {

    type TargetType = String;

    fn name() -> &'static str {
        "invoker-directory-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokerDirectoryName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerDirectoryName::new(s))
    }
}


pub struct InvokerDirectoryServiceName(String);

impl InvokerDirectoryServiceName {

    pub fn new(name: impl Into<String>) -> Self {
        InvokerDirectoryServiceName(name.into())
    }
}

impl UrlParam for InvokerDirectoryServiceName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "invoker-directory-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for InvokerDirectoryServiceName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerDirectoryServiceName::new(s))
    }
}

#[derive(Clone)]
pub struct InvokerDirectoryType(String);

impl InvokerDirectoryType {

    pub fn new(name: impl Into<String>) -> Self {
        InvokerDirectoryType(name.into())
    }
}

impl UrlParam for InvokerDirectoryType {
    
    type TargetType = String;

    fn name() -> &'static str {
        "invoker-directory-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokerDirectoryType {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokerDirectoryType::new(s))
    }
}