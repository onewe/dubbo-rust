use std::str::FromStr;

use dubbo_base::{url::UrlParam, Url};

use crate::{config::{reference_config::{ClusterExtension, InvokerDirectoryExtension}, registry_config::{ReferenceUrl, RegistryUrl}}, extension::{self, protocol_extension::Protocol, ProtocolExtensionLoader}, framework::boot::DubboBoot, invoker::Invoker, protocol::registry_protocol_invoker::RegistryProtocolInvoker, StdError};

pub struct RegistryProtocolLoader;

impl RegistryProtocolLoader {

    pub const NAME: &'static str = "registry";
}

#[async_trait::async_trait]
impl ProtocolExtensionLoader for RegistryProtocolLoader {

   
    fn name(&self) -> String {
        Self::NAME.to_owned()
    }

    async fn load(&mut self, url: &Url) -> Result<Box<dyn Protocol + Send>, StdError> {
        // url: extensions://127.0.0.1:8848?extension-loader-name=registry&extension-name=nacos://127.0.0.1:8848&registry=nacos://127.0.0.1:8848

        // nacos://127.0.0.1:8848
        let registry_url = url.query::<RegistryUrl>().unwrap();

        let mut registry_url = registry_url.value();
        let protocol = registry_url.protocol();
        
        registry_url.add_query_param(RegistryType::new(protocol.to_string()));
        registry_url.set_protocol(RegistryProtocolLoader::NAME);
        
        // registry://127.0.0.1:8848?registry_type=nacos
       Ok(Box::new(RegistryProtocol::new(registry_url)))
    }
}


pub struct RegistryProtocol {
    registry_url: Url
}


impl RegistryProtocol {
    
    pub fn new(registry_url: Url) -> Self {
        Self {
            registry_url,
        }
    }

    
}

#[async_trait::async_trait]
impl Protocol for RegistryProtocol {

    async fn export(&mut self, _: Url) -> Result<(), StdError> {
        Ok(())
    }

    async fn refer(&mut self, reference_url: Url) -> Result<Box<dyn Invoker + Send>, StdError> {
       
        let cluster_extension = reference_url.query::<ClusterExtension>().unwrap();
        let cluster_extension = cluster_extension.value();

        let directory_extension = reference_url.query::<InvokerDirectoryExtension>().unwrap();
        let directory_extension = directory_extension.value();

        
        // registry://127.0.0.1:8848?registry_type=nacos
        let mut dynamic_invoker_directory_url = self.registry_url.clone();
        // dynamic-invoker-directory://127.0.0.1:8848?registry_type=nacos
        dynamic_invoker_directory_url.set_protocol(directory_extension.as_str());

      
        // extensions://127.0.0.1:8848?extension-loader-name=dynamic-invoker-directory&extension-name=dynamic-invoker-directory://127.0.0.1:8848&registry_type=nacos
        let mut invoker_directory_extension_url = extension::build_extension_loader_url(dynamic_invoker_directory_url.host().unwrap(), dynamic_invoker_directory_url.protocol(), dynamic_invoker_directory_url.short_url_without_query().as_str());
        // extensions://127.0.0.1:8848?extension-loader-name=dynamic-invoker-directory&extension-name=dynamic-invoker-directory://127.0.0.1:8848&reference=consumer://xxxxx
        invoker_directory_extension_url.add_query_param(ReferenceUrl::new(reference_url.clone()));
        invoker_directory_extension_url.add_query_param(RegistryUrl::new(self.registry_url.clone()));
        let invoker_directory_extension = DubboBoot::load_invoker_directory_extension(invoker_directory_extension_url).await.unwrap();


        // registry://127.0.0.1:8848?registry_type=nacos
        let mut cluster_url = self.registry_url.clone();
        // cluster://127.0.0.1:8848?registry_type=nacos
        cluster_url.set_protocol(cluster_extension.as_str());
        
       

        // extensions://127.0.0.1:8848?extension-loader-name=cluster&extension-name=cluster://27.0.0.1:8848
        let mut cluster_extension_url = extension::build_extension_loader_url(cluster_url.host().unwrap(), cluster_url.protocol(), cluster_url.short_url_without_query().as_str());
        // extensions://127.0.0.1:8848?extension-loader-name=cluster&extension-name=cluster://27.0.0.1:8848&reference=consumer://xxxxx
        cluster_extension_url.add_query_param(ReferenceUrl::new(reference_url.clone()));
        let cluster_extension = DubboBoot::load_cluster_extension(cluster_extension_url).await.unwrap();
       

        // registry://127.0.0.1:8848?registry_type=nacos
        let mut invoker_url = self.registry_url.clone();
        // registry-invoker://127.0.0.1:8848?registry_type=nacos
        invoker_url.set_protocol(RegistryProtocolInvoker::NAME);
        // registry-invoker://127.0.0.1:8848?registry_type=nacos&reference=consumer://xxxxx
        invoker_url.add_query_param(ReferenceUrl::new(reference_url));



        let invoker = RegistryProtocolInvoker::new(invoker_url, invoker_directory_extension, cluster_extension,);

        Ok(Box::new(invoker))
    }
   
}


pub struct RegistryType(String);

impl RegistryType {
    pub fn new(registry_type: String) -> Self {
        Self(registry_type)
    }
}

impl UrlParam for RegistryType {
    type TargetType = String;

    fn name() -> &'static str {
        "registry-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str<'a>(&'a self) -> std::borrow::Cow<'a, str> {
        self.0.as_str().into()
    }
}

impl FromStr for RegistryType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}