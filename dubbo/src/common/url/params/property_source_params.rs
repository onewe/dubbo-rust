use std::str::FromStr;

use crate::{common::url::UrlParam, StdError};

pub struct PropertySourceName(String);

impl PropertySourceName {

    pub fn new(name: impl Into<String>) -> Self {
        PropertySourceName(name.into())
    }
}


impl UrlParam for PropertySourceName {

    type TargetType = String;

    fn name() -> &'static str {
        "property-source-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> std::borrow::Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for PropertySourceName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PropertySourceName(s.to_string()))
    }
}