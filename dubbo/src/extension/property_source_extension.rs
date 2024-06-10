use std::{collections::HashMap, marker::PhantomData, pin::Pin};

use async_trait::async_trait;
use futures::Future;
use thiserror::Error;
use tokio::sync::watch;

use crate::{common::url::{params::extension_params::{ExtensionName, ExtensionType, ExtensionUrl}, Url, UrlParam}, StdError};

use super::{Extension, ExtensionFactories, ExtensionMetaInfo, LoadExtensionPromise};



// url: property-source://127.0.0.1:8848?property-source-name=nacos-property-source
// extension_url: extension://0.0.0.0?extension-type=property-source&extension-name=nacos-property-source&extension-url=property-source://127.0.0.1:8848?property-source-name=nacos-property-source
#[async_trait]
pub trait PropertySource {

    async fn get_property(&self, key: &str) -> Option<String>;

    async fn properties(&self) -> &HashMap<String, String>;

    async fn ready(&mut self) -> Result<(), StdError>;

    fn order(&self) -> i32;

    fn url(&self) -> &Url;

    fn clone(&self) -> Box<dyn PropertySource + Send + Sync + 'static>;
}

impl Clone for Box<dyn PropertySource + Send + Sync + 'static> {
    fn clone(&self) -> Self {
        PropertySource::clone(self.as_ref())
    }
}


pub trait ConfigProperty {
    
    type Target;
    
    fn create(value: Self::Target) -> Self;

    fn create_from_map(map: &HashMap<String, String>) -> Option<Self> where Self: Sized;

    fn value(&self) -> Self::Target;

    fn into_map(self) -> HashMap<String, String>;
}


pub fn build_property_source_url() -> Url {
    let mut url = Url::empty();
    url.set_protocol("property-source");
    url.set_host("0.0.0.0");
    url
}



#[derive(Default)]
pub struct PropertySourceExtensionLoader {
    factories: HashMap<String, PropertySourceExtensionFactory>,
}


impl PropertySourceExtensionLoader {

    pub fn new() -> Self {
        PropertySourceExtensionLoader {
            factories: HashMap::new(),
        }
    }
}


impl PropertySourceExtensionLoader {


    pub fn register(&mut self, extension_name: String, factory: PropertySourceExtensionFactory) {
        self.factories.insert(extension_name, factory);
    }

    pub fn remove(&mut self, extension_name: &str) {
        self.factories.remove(extension_name);
    }

    pub fn load(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn PropertySource + Send + Sync + 'static>>, StdError> {
        let extension_name = url.query::<ExtensionName>();
        let Some(extension_name) = extension_name else {
            return Err(PropertySourceExtensionLoaderError::new("load property source extension failed, PropertySource extension name mustn't be empty").into());
        };

        let extension_name = extension_name.value();

        let factory = self.factories.get_mut(&extension_name);
        let Some(factory) = factory else {
            return Err(PropertySourceExtensionLoaderError::new(format!("load {} PropertySource extension failed, can not found loadBalancer extension factory", extension_name)).into());
        };
        factory.create(url)
    }
}

type Constructor = fn(Url) -> Pin<Box<dyn Future<Output = Result<Box<dyn PropertySource + Send + Sync + 'static>, StdError>> + Send + 'static>>;

pub struct PropertySourceExtensionFactory {
    constructor: Constructor,
    instances: HashMap<String, LoadExtensionPromise<Box<dyn PropertySource + Send + Sync + 'static>>>,
}


impl PropertySourceExtensionFactory {

    pub fn new(constructor: Constructor) -> Self {
        Self {
            constructor,
            instances: HashMap::default(),
        }
    }
}


impl PropertySourceExtensionFactory {

    pub fn create(&mut self, url: Url) -> Result<LoadExtensionPromise<Box<dyn PropertySource + Send + Sync + 'static>>, StdError> {
        let extension_url = url.query::<ExtensionUrl>();
        let Some(extension_url) = extension_url else {
            return Err(PropertySourceExtensionLoaderError::new("load PropertySource extension failed, PropertySource extension url mustn't be empty").into());
        };

        let extension_url_str = extension_url.as_str();

        let instance = self.instances.get(extension_url_str.as_ref());
        match instance {
            Some(instance) => {
                Ok(instance.clone())
            },
            None => {
                let constructor = self.constructor;

                let creator = move |url: Url| {
                    Box::pin(async move {
                        let property_source = constructor(url).await?;
                        Ok(property_source)
                    }) as Pin<Box<dyn Future<Output = Result<Box<dyn PropertySource + Send + Sync + 'static>, StdError>> + Send + 'static>>
                };

                let promise = LoadExtensionPromise::new(Box::new(creator), extension_url.value());
                self.instances.insert(extension_url_str.to_string(), promise.clone());
                Ok(promise)
            }
        }

    }
}



pub struct PropertySourceExtension<T>(PhantomData<T>)
where
    T: PropertySource + Send + 'static;

impl<T> ExtensionMetaInfo for PropertySourceExtension<T> 
where
    T: Extension<Target = Box<dyn PropertySource + Send + Sync + 'static>>,
    T: PropertySource + Send + 'static
{
    fn name() -> String {
        T::name()
    }

    fn extension_type() -> crate::common::url::params::extension_params::ExtensionType {
        ExtensionType::PropertySource
    }

    fn extension_factory() -> super::ExtensionFactories {
        ExtensionFactories::PropertySourceExtensionFactory(PropertySourceExtensionFactory::new(<T as Extension>::create))
    }
}


#[derive(Debug, Error)]
#[error("load property source extension error: {0}")]
pub struct PropertySourceExtensionLoaderError(String);

impl PropertySourceExtensionLoaderError {
    pub fn new(msg: impl Into<String>) -> Self {
        PropertySourceExtensionLoaderError(msg.into())
    }
}