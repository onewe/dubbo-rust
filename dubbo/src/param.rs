use std::{borrow::Cow, str::FromStr};

use crate::StdError;

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
    CONSUMER,
    PROVIDER
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
        Side::CONSUMER => Cow::Borrowed("consumer"),
        Side::PROVIDER => Cow::Borrowed("provider"),
       }
    }
}

impl FromStr for Side {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "consumer".eq_ignore_ascii_case(s) {
            return Ok(Side::CONSUMER);
        }
        
        if "provider".eq_ignore_ascii_case(s) {
            return Ok(Side::PROVIDER);
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


pub enum Methods {
    Any,
    Methods(Vec<String>)
}