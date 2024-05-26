use std::any::Any;
use std::collections::HashMap;
use std::pin::Pin;

use async_trait::async_trait;
use bytes::Bytes;
use futures::{Future, Stream};
use thiserror::Error;
use crate::params::extension_params::{ExtensionName, ExtensionUrl};
use crate::url::UrlParam;
use crate::{StdError, Url};

use super::LoadExtensionPromise;

// url: protocol://0.0.0.0?protocol-type=dubbo
// extension_url: extension://0.0.0.0?extension-type=protocol&extension-name=dubbo&extension-url=protocol://0.0.0.0?protocol-type=dubbo
#[async_trait]
pub trait Protocol {
    // url: invoker://127.0.0.1:8080?invoker-name=hello_service_invoker&invoker-protocol=trip&invoker-service-name=hello_service
    async fn reference(&mut self, url: Url) -> Result<Box<dyn Invoker + Send + Sync + 'static>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn Protocol + Send + Sync + 'static>;

}


impl Clone for Box<dyn Protocol + Send + Sync + 'static> {
        
    fn clone(&self) -> Self {
        Protocol::clone(self.as_ref())
    }
}

pub fn build_protocol_url() -> Url {
    let mut url = Url::empty();
    url.set_protocol("protocol");
    url.set_host("0.0.0.0");
    url
}


// invoker://127.0.0.1:8080?invoker-name=hello-service-invoker&invoker-protocol=trip&invoker-service-name=hello_service
#[async_trait]
pub trait Invoker {
    async fn invoke(
        &mut self,
        invocation: GrpcInvocation,
    ) -> Result<Pin<Box<dyn Stream<Item = Bytes> + Send + 'static>>, StdError>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn url(&self) -> Result<Url, StdError>;

    fn clone(&self) -> Box<dyn Invoker + Send + Sync + 'static>;
}

impl Clone for Box<dyn Invoker + Send + Sync + 'static> {
    fn clone(&self) -> Box<dyn Invoker + Send + Sync + 'static> {
        Invoker::clone(self.as_ref())
    }
}

pub enum CallType {
    Unary,
    ClientStream,
    ServerStream,
    BiStream,
}

pub struct GrpcInvocation {
    service_name: String,
    method_name: String,
    arguments: Vec<Argument>,
    attachments: HashMap<String, String>,
    call_type: CallType,
}

pub struct Argument {
    name: String,
    value: Box<dyn Stream<Item = Box<dyn Serializable + Send + 'static>> + Send + 'static>,
}

pub trait Serializable {
    fn serialize(&self, serialization_type: String) -> Result<Bytes, StdError>;

    fn into_any(self) -> Box<dyn Any + Send + 'static>;
}

pub trait Deserializable {
    fn deserialize(&self, bytes: Bytes, deserialization_type: String) -> Result<Self, StdError>
    where
        Self: Sized;
}


#[derive(Default)]
pub struct ProtocolExtensionLoader {
    factories: HashMap<String, ProtocolExtensionFactory>,
}


impl ProtocolExtensionLoader {

    pub fn new() -> Self {
        ProtocolExtensionLoader {
            factories: HashMap::new(),
        }
    }
}

impl ProtocolExtensionLoader {


    pub fn register(&mut self, extension_name: String, factory: ProtocolExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }


    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }


    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Protocol + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();

        let Some(extension_name) = extension_name else {
            return Err(ProtocolExtensionLoaderError::new("load protocol extension failed, protocol extension name mustn't be empty").into());
        };

        let extension_name = extension_name.value();

        let factory = self.factories.get_mut(&extension_name);

        let Some(factory) = factory else {
            return Err(ProtocolExtensionLoaderError::new(format!("load {} protocol extension failed, can not found router extension factory", extension_name)).into());
        };

        factory.create(url)

    }
}

type Constructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn Protocol + Send + Sync + 'static>, StdError>> + Send + 'static>>;
pub struct ProtocolExtensionFactory {
    constructor: Constructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn Protocol + Send + Sync + 'static>>>,
}


impl ProtocolExtensionFactory {

    pub fn new(constructor: Constructor) -> Self {
        ProtocolExtensionFactory {
            constructor,
            instances: HashMap::new(),
        }
    }
}


impl ProtocolExtensionFactory {

    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn Protocol + Send + Sync + 'static>>, StdError> {

        let extension_url = url.query::<ExtensionUrl>();
        let Some(extension_url) = extension_url else {
            return Err(ProtocolExtensionLoaderError::new("load protocol extension failed, protocol extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();

        let instance = self.instances.get(extension_url_str.as_ref());
        match instance {
            Some(instance) => Ok(instance.clone()),
            None => {
                let constructor = self.constructor;
                let creator = move |url: Url| {
                    Box::pin(async move {
                        let protocol = constructor(url).await?;
                        Ok(protocol)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn Protocol + Send + Sync + 'static>, StdError>> + Send + 'static>>
                }; 

                let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
                self.instances.insert(extension_url_str.to_string(), promise.clone());
                Ok(promise)
            }
        }
    }
}


#[derive(Debug, Error)]
#[error("load protocol extension error: {0}")]
pub struct ProtocolExtensionLoaderError(String);


impl ProtocolExtensionLoaderError {

    pub fn new(msg: impl Into<String>) -> Self {
        ProtocolExtensionLoaderError(msg.into())
    }
}