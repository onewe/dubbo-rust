use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use dubbo_base::{url::UrlParam, Url};
use thiserror::Error;
use tokio::sync::watch;

use crate::{config::registry_config::{ReferenceUrl, RegistryUrl}, extension::{self, directory_extension::InvokerDirectory, registry_extension::Registry, InvokerDirectoryExtensionLoader}, framework::boot::DubboBoot, invoker::{cloneable_invoker::CloneableInvoker, Invoker}, protocol::registry_protocol_extension::RegistryType, StdError};

pub struct DynamicInvokerDirectoryLoader;

impl DynamicInvokerDirectoryLoader {

    pub const EXTENSION_NAME: &'static str = "dynamic-invoker-directory";
}

#[async_trait::async_trait]
impl InvokerDirectoryExtensionLoader for DynamicInvokerDirectoryLoader {

    fn name(&self) -> String {
        Self::EXTENSION_NAME.to_string()
    }

    async fn load(&mut self, url: &Url) -> Result<Box<dyn InvokerDirectory + Send>, StdError> {
       
        //extensions://127.0.0.1:8848?extension-loader-name=dynamic-invoker-directory&extension-name=dynamic-invoker-directory://127.0.0.1:8848&registry_type=nacos
        let registry_url = url.query::<RegistryUrl>().unwrap();
        let reference_url = url.query::<ReferenceUrl>().unwrap();

        // registry://127.0.0.1:8848?registry_type=nacos
        let mut registry_url = registry_url.value();
        let registry_type = registry_url.query::<RegistryType>().unwrap();
        let registry_type = registry_type.value();

        // nacos://127.0.0.1:8848
        registry_url.set_protocol(registry_type.as_str());
        registry_url.remove_query_param::<RegistryType>();

        let mut registry_extension_url = extension::build_extension_loader_url(registry_url.host().unwrap(), registry_url.protocol(), registry_url.short_url_without_query().as_str());
        registry_extension_url.add_query_param(RegistryUrl::new(registry_url));

        let registry_extension = DubboBoot::load_registry_extension(registry_extension_url).await.unwrap();


        let directory = DynamicInvokerDirectory::new(reference_url.value(), registry_extension);
        
        Ok(Box::new(directory))
    }
}

pub struct DynamicInvokerDirectory {
    reference_url: Url,
    registry: Box<dyn Registry + Send>,
    subscriber: Option<watch::Receiver<std::collections::HashSet<Url>>>,
    invoker_cache: HashMap<String, CloneableInvoker>,
}


impl DynamicInvokerDirectory {

    pub fn new(reference_url: Url, registry: Box<dyn Registry + Send>) -> Self {

        Self {
            reference_url,
            registry,
            subscriber: None,
            invoker_cache: HashMap::new(),
        }
    }
}

impl DynamicInvokerDirectory {

   async fn to_invokers(urls: HashSet<Url>) -> HashMap<String, CloneableInvoker> {
        let mut invokers = HashMap::new();
        
        for invoker_url in urls {
            // extension://127.0.0.1:9999?extension-loader-name=dubbo&extension-name=dubbo://127.0.0.1
            let extension_url = extension::build_extension_loader_url(invoker_url.host().unwrap(), invoker_url.protocol(), invoker_url.short_url_without_query().as_str());
            
            let mut protocol = DubboBoot::load_protocol_extension(extension_url).await.unwrap();
            
            let invoker = protocol.refer(invoker_url.clone()).await.unwrap();
            
            invokers.insert(invoker_url.as_str().to_owned(), CloneableInvoker::new(invoker));
        }

        invokers
    }
    
}


#[async_trait]
impl InvokerDirectory for DynamicInvokerDirectory {

    async fn list(&mut self) -> Result<Vec<Box<dyn Invoker + Send>>, StdError> {

        if self.subscriber.is_none() {
            let subscriber = self.registry.subscribe(self.reference_url.clone()).await?;
            self.subscriber = Some(subscriber);
        }

        match self.subscriber {
            Some(ref mut subscriber) => {
                
                let changed = subscriber.has_changed()?;
                if changed {
                    let changes = subscriber.borrow_and_update().clone();
                    let invokers = Self::to_invokers(changes).await;

                    self.invoker_cache.extend(invokers);
                } else {
                    if self.invoker_cache.is_empty() {
                        let _ = subscriber.changed().await?;
                        let changes = subscriber.borrow_and_update().clone();
                        let invokers = Self::to_invokers(changes).await;

                        self.invoker_cache.extend(invokers);
                    } 
                }

                let invokers = self.invoker_cache.values().map(|invoker| Box::new(invoker.clone()) as Box<dyn Invoker + Send>).collect();
                Ok(invokers)
            },
            None => {
                Err(SubscribeError::new("subscriber is none").into())
            },
        }

    }
}


#[derive(Error, Debug)]
#[error("dynamic invoker directory error: {0}")]
pub struct SubscribeError(String);

impl SubscribeError {

    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
    
}