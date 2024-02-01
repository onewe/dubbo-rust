use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use dubbo_base::{url::UrlParam, Url};
use nacos_sdk::api::{naming::{NamingService, NamingServiceBuilder, ServiceInstance}, props::ClientProps};
use tokio::sync::watch;

use crate::{config::{reference_config::{Category, Group, InterfaceName, Version}, registry_config::{AppName, RegistryUrl, ServiceNamespace}}, extension::{registry_extension::Registry, RegistryExtensionLoader}, StdError};


pub struct NacosRegistryLoader;

impl NacosRegistryLoader {

    pub const NAME: &'static str = "nacos-registry";
}

#[async_trait::async_trait]
impl RegistryExtensionLoader for NacosRegistryLoader {

    fn name(&self) -> String {
        Self::NAME.to_string()
    }

    async fn load(&mut self, url: &Url) -> Result<Box<dyn Registry + Send>, StdError> {
        let registry_url = url.query::<RegistryUrl>().unwrap();
        let registry_url = registry_url.value();


        let host = registry_url.host().unwrap();
        let port = registry_url.port().unwrap_or(8848);

        let nacos_server_addr = format!("{}:{}", host, port);


        let namespace = url.query::<ServiceNamespace>().unwrap_or_default();
        let namespace = namespace.value();

        let app_name = url.query::<AppName>().unwrap_or_default();
        let app_name = app_name.value();


        let user_name = url.username();
        let password = url.password().unwrap_or_default();


        let nacos_client_props = ClientProps::new()
                        .server_addr(nacos_server_addr)
                        .namespace(namespace)
                        .app_name(app_name)
                        .auth_username(user_name)
                        .auth_password(password);

        let mut nacos_naming_builder = NamingServiceBuilder::new(nacos_client_props);

        if !user_name.is_empty() {
            nacos_naming_builder = nacos_naming_builder.enable_auth_plugin_http();
        }
        
        let nacos_naming_service = nacos_naming_builder.build().unwrap();

        let nacos_registry = NacosRegistry::new(registry_url, Arc::new(nacos_naming_service));

        Ok(Box::new(nacos_registry))
    }

}


pub struct NacosRegistry {
    url: Url,
    nacos_service: Arc<dyn NamingService + Send + Sync>
}

impl NacosRegistry {

    pub fn new(url: Url, nacos_service: Arc<dyn NamingService + Send + Sync>) -> Self {
        Self {
            url,
            nacos_service
        }
    }

    fn create_nacos_service_instance(url: &Url) -> ServiceInstance {
       let ip = url.host().unwrap();
       let port = url.port().unwrap();

       ServiceInstance {
              ip: ip.to_string(),
              port: port.into(),
              metadata: url.all_query_params(),
              ..Default::default()
       } 

    }
}

#[async_trait]
impl Registry for NacosRegistry {

    async fn register(&mut self, url: Url) -> Result<(), StdError> {
        let service_name = NacosServiceName::new(&url);

        let group_name = service_name.group();

        let registry_service_name_str= service_name.value();

        let service_instance = Self::create_nacos_service_instance(&url);

        self.nacos_service.register_instance(registry_service_name_str.to_owned(), Some(group_name.to_owned()), service_instance).await?;

        Ok(())
    }

    async fn unregister(&mut self, url: Url) -> Result<(), StdError> {
        let service_name = NacosServiceName::new(&url);

        let group_name = service_name.group();

        let registry_service_name_str= service_name.value();

        let service_instance = Self::create_nacos_service_instance(&url);

        self.nacos_service.deregister_instance(registry_service_name_str.to_owned(), Some(group_name.to_owned()), service_instance).await?;
        
        Ok(())
    }

    async fn subscribe(&mut self, url: Url) -> Result<watch::Receiver<HashSet<String>>, StdError> {

        todo!()
    }

    fn url(&self) -> &Url {
        &self.url
    }

}

struct NacosServiceName {

    category: String,

    interface: String,

    version: String,

    group: String,

    value: String,

}


impl NacosServiceName {

    fn new(url: &Url) -> Self {

        let interface = url.query::<InterfaceName>().unwrap();
        let interface = interface.value();

        let category = url.query::<Category>().unwrap();
        let category = category.value();

        let version = url.query::<Version>().unwrap_or_default();
        let version = version.value();

        let group = url.query::<Group>().unwrap_or_default();
        let group = group.value();

        let value = format!("{}:{}:{}:{}", category, interface, version, group);

        Self { category, interface, version, group, value }
    }

    fn category(&self) -> &str {
        &self.category
    }

    fn interface(&self) -> &str {
        &self.interface
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn group(&self) -> &str {
        &self.group
    }

    fn value(&self) -> &str {
        &self.value
    }
}