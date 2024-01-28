use std::str::FromStr;

use dubbo_base::{url::UrlParam, Url};

use crate::StdError;

pub struct RegistryConfig {
    inner: Url
}


impl RegistryConfig {

    pub fn empty() -> Self {
        let empty_url = "registry://empty".parse().unwrap();
        Self {
            inner: empty_url,
        }
    }

    pub fn new(url: &str) -> Result<Self, StdError> {
        let url =  url.parse()?;
        Ok(Self {
            inner: url,
        })
    }

    pub fn url(&self) -> &Url {
        &self.inner
    }

}


impl FromStr for RegistryConfig {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url =  s.parse()?;
        Ok(Self {
            inner: url,
        })
    }
}

pub struct ReferenceUrl(Url);

impl ReferenceUrl {
    
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}


impl UrlParam for ReferenceUrl {
    type TargetType = Url;

    fn name() -> &'static str {
        "reference"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for ReferenceUrl {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}