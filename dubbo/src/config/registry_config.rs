use std::{borrow::Cow, str::FromStr};

use crate::{url::Url, param::Param, StdError};

pub struct RegistryConfig {
    url: Url
}

impl RegistryConfig {

    pub fn new(url: Url) -> Self {
        Self {
            url
        }
    }

    pub fn protocol(&self) -> &str {
        self.url.protocol()
    }

    pub fn host(&self) -> Option<&str> {
        self.url.host()
    }

    pub fn username(&self) -> &str {
        self.url.username()
    }

    pub fn password(&self) -> Option<&str> {
        self.url.password()
    }

    pub fn port(&self) -> Option<u16> {
        self.url.port()
    }

    pub fn path(&self) -> &str {
        self.url.path()
    }

    pub fn query<T: Param>(&self) -> Option<T> {
        self.url.query()
    }

    pub fn set_protocol(&mut self, protocol: &str) {
        self.url.set_protocol(protocol);
    }


    pub fn set_host(&mut self, host: &str) {
        self.url.set_host(host);
    }

    pub fn set_port(&mut self, port: u16) {
        self.url.set_port(port);
    }

    pub fn set_username(&mut self, username: &str) {
        self.url.set_username(username);
    }

    pub fn set_password(&mut self, password: &str) {
        self.url.set_password(password);
    }

    pub fn set_path(&mut self, path: &str) {
        self.url.set_path(path);
    }

    pub fn add_query_param<T: Param>(&mut self, param: &T) {
        self.url.add_query_param(param);
    }

}


pub struct UseAsConfigCenter(bool);

impl Param for UseAsConfigCenter {

    type TargetType = bool;

    fn name() -> &'static str {
        "useAsConfigCenter"
    }


    fn value(&self) -> Self::TargetType {
        self.0
    }

    fn as_str(&self) -> Cow<'static, str> {
        match self.0 {
            true => Cow::Borrowed("true"),
            false => Cow::Borrowed("false")
        }
    }
}

impl FromStr for UseAsConfigCenter {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let enable = s.parse::<bool>()?;
        Ok(Self(enable))
    }
}

pub struct UseAsMetadataCenter(bool);

impl Param for UseAsMetadataCenter {

    type TargetType = bool;

    fn name() -> &'static str {
        "useAsMetadataCenter"
    }

    fn value(&self) -> Self::TargetType {
        self.0
    }

    fn as_str(&self) -> Cow<'static, str> {
        match self.0 {
            true => Cow::Borrowed("true"),
            false => Cow::Borrowed("false")
        }
    }
}

impl FromStr for UseAsMetadataCenter {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let enable = s.parse::<bool>()?;
        Ok(Self(enable))
    }
}

pub struct EnableEmptyProtection(bool);

impl Param for EnableEmptyProtection {

    type TargetType = bool;

    fn name() -> &'static str {
        "enableEmptyProtection"
    }


    fn value(&self) -> Self::TargetType {
        self.0
    }

    fn as_str(&self) -> Cow<'static, str> {
        match self.0 {
            true => Cow::Borrowed("true"),
            false => Cow::Borrowed("false")
        }
    }
}

impl FromStr for EnableEmptyProtection {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let enable = s.parse::<bool>()?;
        Ok(Self(enable))
    }
}