use std::{borrow::Cow, str::FromStr};

use crate::{StdError, url::Url};

pub trait Param: FromStr {

    type TargetType;

    fn name() -> &'static str;

    fn value(&self) -> Self::TargetType;

    fn as_str<'a>(&'a self) -> Cow<'a, str>;

}


pub struct Interface(String);

impl Param for Interface {

    type TargetType = String;

    fn name() -> &'static str {
        "interface"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        Cow::Borrowed(&self.0)
    }
}

impl FromStr for Interface {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Interface(s.to_owned()))
    }
}


#[derive(Clone, Copy)]
pub enum Side {
    Consumer,
    Provider
}


impl Param for Side {
        
    type TargetType = Self;

    fn name() -> &'static str {
        "side"
    }

    fn value(&self) -> Self::TargetType {
       self.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
       match self {
        Side::Consumer => Cow::Borrowed("consumer"),
        Side::Provider => Cow::Borrowed("provider"),
       }
    }
}

impl FromStr for Side {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "consumer".eq_ignore_ascii_case(s) {
            return Ok(Side::Consumer);
        }
        
        if "provider".eq_ignore_ascii_case(s) {
            return Ok(Side::Provider);
        }

        panic!("invalid side: {}", s)
    }
}


pub struct Revision(String);

impl Param for Revision {

    type TargetType = String;

    fn name() -> &'static str {
        "revision"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        Cow::Borrowed(&self.0)
    }
}

impl FromStr for Revision {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Revision(s.to_owned()))
    }
}

#[derive(Clone)]
pub enum Methods {
    Any,
    Methods(Vec<String>)
}

impl Param for Methods {

    type TargetType = Self;

    fn name() -> &'static str {
        "methods"
    }

    fn value(&self) -> Self::TargetType {
        self.clone()
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        match self {
            Methods::Any => Cow::Borrowed("*"),
            Methods::Methods(methods) => Cow::Owned(methods.join(","))
        }
    }
}

impl FromStr for Methods {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "*".eq_ignore_ascii_case(s) {
            return Ok(Methods::Any);
        }

        let methods = s.split(",").map(|s| s.to_owned()).collect::<Vec<_>>();
        Ok(Methods::Methods(methods))
    }
}

pub enum RegisterIp {
    LocalHost,
    Ip(String)
}


impl Param for RegisterIp {
            
        type TargetType = String;
    
        fn name() -> &'static str {
            "register.ip"
        }
    
        fn value(&self) -> Self::TargetType {
           match self {
            RegisterIp::LocalHost => "127.0.0.1".to_owned(),
            RegisterIp::Ip(ip) => ip.clone(),
           }
        }
    
        fn as_str<'a>(&'a self) -> Cow<'a, str> {
            match self {
                RegisterIp::LocalHost => Cow::Borrowed("127.0.0.1"),
                RegisterIp::Ip(ip) => Cow::Borrowed(ip),
            }
        
        }

}

impl FromStr for RegisterIp {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(RegisterIp::LocalHost);
        }
        Ok(RegisterIp::Ip(s.to_owned()))
    }
}

pub struct Registry(String);

impl Param for Registry {

    type TargetType = String;

    fn name() -> &'static str {
        "registry"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        Cow::Borrowed(&self.0)
    }
}

impl FromStr for Registry {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Registry(s.to_owned()))
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

#[derive(Clone)]
pub struct ReferenceUrl(Url);

impl ReferenceUrl {

    pub fn new(url: Url) -> Self {
        Self(url)
    }
}

impl Param for ReferenceUrl {

    type TargetType = Url;

    fn name() -> &'static str {
        "reference.url"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self.0.as_str())
    }
}

impl FromStr for ReferenceUrl {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse()?;
        Ok(Self(url))
    }
}

pub struct Extension(String);

impl Param for Extension {

    type TargetType = String;

    fn name() -> &'static str {
        "extension"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        Cow::Borrowed(&self.0)
    }
}

impl FromStr for Extension {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Extension(s.to_owned()))
    }
}