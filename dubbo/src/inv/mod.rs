use std::fmt::Display;

use bytes::Bytes;
use futures::Stream;

use crate::{StdError, url::Url};

#[derive(Default)]
pub(crate) struct RpcInvocation {

    service_name: String,

    interface_name: String,

    method_name: String,

    arguments: Vec<Argument>,
}

impl RpcInvocation {
    
        pub(crate) fn new(service_name: String, interface_name: String, method_name: String, arguments: Vec<Argument>) -> Self {
            Self {
                service_name,
                interface_name,
                method_name,
                arguments,
            }
        }

        pub(crate) fn service_name(&self) -> &str {
            &self.service_name
        }

        pub(crate) fn interface_name(&self) -> &str {
            &self.interface_name
        }


        pub(crate) fn method_name(&self) -> &str {
            &self.method_name
        }

        pub(crate) fn arguments(&self) -> &[Argument] {
            &self.arguments
        }

        pub(crate) fn set_service_name(&mut self, service_name: String) {
            self.service_name = service_name;
        }

        pub(crate) fn set_interface_name(&mut self, interface_name: String) {
            self.interface_name = interface_name;
        }

        pub(crate) fn set_method_name(&mut self, method_name: String) {
            self.method_name = method_name;
        }

        pub(crate) fn set_arguments(&mut self, arguments: Vec<Argument>) {
            self.arguments = arguments;
        }

        pub(crate) fn add_argument(&mut self, argument: Argument) {
            self.arguments.push(argument);
        }

        
}


pub(crate) struct RpcResponse {
    
    data: Box<dyn Serializable>,
}

pub(crate) struct Argument {

    name: String,

    data: Box<dyn Serializable>
}

impl Argument {
    
        pub(crate) fn new(name: String, data: Box<dyn Serializable>) -> Self {
            Self {
                name,
                data,
            }
        }
    
        pub(crate) fn name(&self) -> &str {
            &self.name
        }
    
        pub(crate) fn data(&self) -> &dyn Serializable {
            self.data.as_ref()
        }
    
}

pub(crate) trait Serializable: Display {

    fn serialize(&self) -> Result<Box<dyn Stream<Item = Bytes>>, StdError>;
}

pub(crate) trait Deserializable<T> {
    
    
    fn deserialize(&self, data: Box<dyn Stream<Item = Bytes>>) -> Result<T, StdError>;
    
}


#[async_trait::async_trait]
pub(crate) trait Invoker {

    async fn invoke(&mut self, invocation: &RpcInvocation) -> Result<RpcResponse, StdError>;

    fn url(&self) -> &Url;
    
}