//invoke-url: invoke://service-name/invoke-service-name=hello&invoke-method-name=sayHello&invoke-call-type=Unary

use std::{borrow::Cow, str::FromStr};

use crate::{url::UrlParam, StdError};



pub struct InvokeServiceName(String);

impl InvokeServiceName {
    pub fn new(name: impl Into<String>) -> Self {
        InvokeServiceName(name.into())
    }
}

impl UrlParam for InvokeServiceName {
    type TargetType = String;

    fn name() -> &'static str {
        "invoke-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for InvokeServiceName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokeServiceName::new(s.to_string()))
    }
}


pub struct InvokeMethodName(String);

impl InvokeMethodName {
    pub fn new(name: impl Into<String>) -> Self {
        InvokeMethodName(name.into())
    }
}


impl UrlParam for InvokeMethodName {

    type TargetType = String;

    fn name() -> &'static str {
        "invoke-method-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for InvokeMethodName {

    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(InvokeMethodName::new(s.to_string()))
    }
}


pub enum InvokeCallType {
    Unary,
    ClientStream,
    ServerStream,
    BiStream,
}


impl UrlParam for InvokeCallType {
    type TargetType = String;

    fn name() -> &'static str {
        "invoke-call-type"
    }

    fn value(&self) -> Self::TargetType {
        match self {
            InvokeCallType::Unary => "Unary".to_string(),
            InvokeCallType::ClientStream => "ClientStream".to_string(),
            InvokeCallType::ServerStream => "ServerStream".to_string(),
            InvokeCallType::BiStream => "BiStream".to_string(),
        }
    }

    fn as_str(&self) -> Cow<str> {
       match self {
           InvokeCallType::Unary => "Unary".into(),
           InvokeCallType::ClientStream => "ClientStream".into(),
           InvokeCallType::ServerStream => "ServerStream".into(),
           InvokeCallType::BiStream => "BiStream".into(),
       }
    }
}

impl FromStr for InvokeCallType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "unary" => Ok(InvokeCallType::Unary),
            "clientstream" => Ok(InvokeCallType::ClientStream),
            "serverstream" => Ok(InvokeCallType::ServerStream),
            "bistream" => Ok(InvokeCallType::BiStream),
            _ => Err(StdError::from("Invalid InvokeCallType")),
        }
    }
}




