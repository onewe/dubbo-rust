use std::fmt::Display;

use bytes::Bytes;
use dubbo_base::Url;
use futures::Stream;
use thiserror::Error;

use crate::StdError;

pub mod cloneable_invoker;


pub trait Serializable: Display {

    fn serialize(&self) -> Result<Box<dyn Stream<Item = Bytes> + Send>, StdError>;
}


pub trait Deserializable {
    
    fn deserialize<T>(&self, data: Box<dyn Stream<Item = Bytes> + Send>) -> Result<T, StdError>;
}



pub struct Argument {

    name: String,

    data: Box<dyn Serializable + Send + 'static>
}

impl Argument {
    
        pub fn new(name: String, data: Box<dyn Serializable + Send + 'static>) -> Self {
            Self {
                name,
                data,
            }
        }
    
        pub fn name(&self) -> &str {
            &self.name
        }
    
        pub fn data(&self) -> &dyn Serializable {
            self.data.as_ref()
        }
    
}


pub struct RpcResponse {
    
    data: Box<dyn Stream<Item = Bytes> + Send>,
}


#[derive(Default)]
pub struct RpcInvocation {

    service_name: String,

    interface_name: String,

    method_name: String,

    arguments: Vec<Argument>,
}



impl RpcInvocation {
    
    pub fn new(service_name: String, interface_name: String, method_name: String, arguments: Vec<Argument>) -> Self {
        Self {
            service_name,
            interface_name,
            method_name,
            arguments,
        }
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn interface_name(&self) -> &str {
        &self.interface_name
    }


    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }

    pub fn set_service_name(&mut self, service_name: String) {
        self.service_name = service_name;
    }

    pub fn set_interface_name(&mut self, interface_name: String) {
        self.interface_name = interface_name;
    }

    pub fn set_method_name(&mut self, method_name: String) {
        self.method_name = method_name;
    }

    pub fn set_arguments(&mut self, arguments: Vec<Argument>) {
        self.arguments = arguments;
    }

    pub fn add_argument(&mut self, argument: Argument) {
        self.arguments.push(argument);
    }
    
}




#[async_trait::async_trait]
pub trait Invoker {

    async fn ready(&mut self) -> Result<(), StdError>;

    async fn invoke(&mut self, invocation: RpcInvocation) -> Result<RpcResponse, StdError>;

    fn url(&self) -> &Url;
    
}


#[derive(Error, Debug)]
#[error("invoke error: {0}")]
pub struct InvokeError(String);

impl InvokeError {   
    pub fn new(msg: String) -> Self {
        Self(msg)
    }
    
}