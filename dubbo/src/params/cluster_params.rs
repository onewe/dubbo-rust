use std::borrow::Cow;
use std::str::FromStr;
use crate::StdError;
use crate::url::UrlParam;

pub struct ClusterType(String);

impl ClusterType {
    pub fn new(name: String) -> Self {
        ClusterType(name)
    }
}

impl UrlParam for ClusterType {
    type TargetType = String;

    fn name() -> &'static str {
        "cluster"
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