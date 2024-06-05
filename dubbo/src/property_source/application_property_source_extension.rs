use std::collections::HashMap;
use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::watch;

use crate::{common::url::{params::{cluster_params::ClusterType, invoker_direcotry_params::InvokerDirectoryType, load_balancer_params::LoadBalancerType, property_source_params::{PropertySourceName, PropertySourceUrl}, registry_params::RegistryUrl, router_params::RouterType}, Url, UrlParam}, config::{cluster_config::ClusterConfig, DubboConfig}, extension::property_source_extension::{self, PropertySource}, StdError};


pub struct ApplicationPropertySourceExtension {
    properties: HashMap<String, String>,
    order: i32,
}


impl ApplicationPropertySourceExtension {
    
    pub fn new() -> Self {
        ApplicationPropertySourceExtension {
            properties: HashMap::new(),
            order: 0,
        }
    }
}

impl Clone for ApplicationPropertySourceExtension {
    fn clone(&self) -> Self {
        Self { properties: self.properties.clone(), order: self.order.clone() }
    }
}


#[async_trait]
impl PropertySource for ApplicationPropertySourceExtension {


    async fn get_property(&self, key: &str) -> Option<String> {
        self.properties.get(key).cloned()
    }

    async fn watch(&mut self, _: &str) -> Result<watch::Receiver<String>, StdError> {
        Err(ApplicationPropertySourceError::new("ApplicationPropertySourceExtension does not support watch").into())
    }

    async fn ready(&mut self) -> Result<(), StdError> {
        Ok(())
    }

    fn order(&self) -> i32 {
        self.order
    }

    fn url(&self) -> &Url {
        todo!()
    }

    fn clone(&self) -> Box<dyn PropertySource + Send + Sync + 'static> {
        let property_source = Clone::clone(self);
        Box::new(property_source)
    }   
}





#[derive(Debug, Error)]
#[error("ApplicationPropertySourceError: {0}")]
pub struct ApplicationPropertySourceError(String);

impl ApplicationPropertySourceError {

    pub fn new(message: impl Into<String>) -> Self {
        ApplicationPropertySourceError(message.into())
    }
}



impl DubboConfig {

    pub fn into_application_config_url(self) -> Url {

        let mut url = property_source_extension::build_property_source_url();

        // add cluster url param
        let cluster_type = ClusterType::new(self.cluster_config.cluster_type());
        url.add_query_param(cluster_type);

        // add invoker directory param
        let invoker_directory_type = InvokerDirectoryType::new(self.invoker_directory_config.invoker_directory_type());
        url.add_query_param(invoker_directory_type);

        // add load balancer param
        let load_balancer_type = LoadBalancerType::new(self.load_balancer_config.load_balancer_type());
        url.add_query_param(load_balancer_type);

        // add route type
        let route_type = RouterType::new(self.route_config.route_type());
        url.add_query_param(route_type);

        // add resource url param
        let property_source_url = PropertySourceUrl::new(self.property_source_config.url().clone());
        url.add_query_param(property_source_url);

        // add registry url param
        let registry_url = RegistryUrl::new(self.registry_config.url().clone());
        url.add_query_param(registry_url);

        // add property source name
        let property_source_name = PropertySourceName::new("application-property-source");
        url.add_query_param(property_source_name);

        url

    }
}


pub trait Properties {

    fn as_map(&self) -> HashMap<String, String>;

}