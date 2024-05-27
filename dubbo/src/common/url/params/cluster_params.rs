use std::borrow::Cow;
use std::str::FromStr;
use crate::common::url::UrlParam;
use crate::StdError;

#[derive(Debug, Clone)]
pub struct ClusterType(String);

impl ClusterType {
    pub fn new(name: impl Into<String>) -> Self {
        ClusterType(name.into())
    }
}

impl UrlParam for ClusterType {
    type TargetType = String;

    fn name() -> &'static str {
        "cluster-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for ClusterType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClusterType::new(s.to_string()))
    }
}


#[derive(Debug, Clone)]
pub struct ClusterName(String);


impl ClusterName {
    pub fn new(name: impl Into<String>) -> Self {
        ClusterName(name.into())
    }
}


impl UrlParam for ClusterName {

    type TargetType = String;

    fn name() -> &'static str {
        "cluster-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for ClusterName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClusterName::new(s.to_string()))
    }
}


#[derive(Debug, Clone)]
pub struct ClusterServiceName(String);


impl ClusterServiceName {
    pub fn new(name: impl Into<String>) -> Self {
        ClusterServiceName(name.into())
    }
}


impl UrlParam for ClusterServiceName {

    type TargetType = String;

    fn name() -> &'static str {
        "cluster-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for ClusterServiceName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ClusterServiceName::new(s.to_string()))
    }
}
