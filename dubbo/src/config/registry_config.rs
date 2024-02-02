use std::str::FromStr;

use dubbo_base::{url::UrlParam, Url};

use crate::StdError;

pub struct RegistryConfig {
    inner: Url,
}

impl RegistryConfig {
    pub fn empty() -> Self {
        let empty_url = "registry://empty".parse().unwrap();
        Self { inner: empty_url }
    }

    pub fn new(url: &str) -> Result<Self, StdError> {
        let url = url.parse()?;
        Ok(Self { inner: url })
    }

    pub fn url(&self) -> &Url {
        &self.inner
    }
}

impl FromStr for RegistryConfig {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse()?;
        Ok(Self { inner: url })
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

pub struct RegistryUrl(Url);

impl RegistryUrl {
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}

impl UrlParam for RegistryUrl {
    type TargetType = Url;

    fn name() -> &'static str {
        "registry"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for RegistryUrl {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

pub struct ServiceNamespace(String);

impl ServiceNamespace {
    pub fn new(namespace: String) -> Self {
        Self(namespace)
    }
}

impl UrlParam for ServiceNamespace {
    type TargetType = String;

    fn name() -> &'static str {
        "namespace"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for ServiceNamespace {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Default for ServiceNamespace {
    fn default() -> Self {
        Self("public".to_string())
    }
}

pub struct AppName(String);

impl AppName {
    pub fn new(app_name: String) -> Self {
        Self(app_name)
    }
}

impl UrlParam for AppName {
    type TargetType = String;

    fn name() -> &'static str {
        "app_name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for AppName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Default for AppName {
    fn default() -> Self {
        Self("UnknownApp".to_string())
    }
}

pub struct ServiceProtocol(String);

impl ServiceProtocol {
    pub fn new(protocol: String) -> Self {
        Self(protocol)
    }
}

impl UrlParam for ServiceProtocol {
    type TargetType = String;

    fn name() -> &'static str {
        "protocol"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for ServiceProtocol {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Default for ServiceProtocol {
    fn default() -> Self {
        Self("dubbo".to_string())
    }
}

