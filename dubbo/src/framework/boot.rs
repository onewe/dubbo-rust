use std::collections::HashMap;

use dubbo_base::{url::UrlParam, Url};

use crate::{
    config::{
        reference_config::{ReferenceConfig, RustTypeName},
        registry_config::{ReferenceUrl, RegistryConfig, RegistryUrl},
    },
    extension::{
        self, cluster_extension, directory_extension, load_balance_extension,
        protocol_extension::Protocol, registry_extension, router_extension,
        ProtocolExtensionLoader,
    },
    invoker::cloneable_invoker::CloneableInvoker,
    StdError,
};

use super::Dubbo;

pub struct DubboBoot {
    reference_configs: Vec<ReferenceConfig>,

    registry_configs: Vec<RegistryConfig>,
}

impl DubboBoot {
    pub async fn add_protocol_extension_loader(
        loader: Box<dyn ProtocolExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_protocol_extension_loader(loader)
            .await
    }

    pub async fn add_registry_extension_loader(
        loader: Box<dyn extension::RegistryExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_registry_extension_loader(loader)
            .await
    }

    pub async fn add_cluster_extension_loader(
        loader: Box<dyn extension::ClusterExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_cluster_extension_loader(loader)
            .await
    }

    pub async fn add_load_balance_extension_loader(
        loader: Box<dyn extension::LoadBalanceExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_load_balance_extension_loader(loader)
            .await
    }

    pub async fn add_router_extension_loader(
        loader: Box<dyn extension::RouterExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_router_extension_loader(loader)
            .await
    }

    pub async fn add_invoker_directory_extension_loader(
        loader: Box<dyn extension::InvokerDirectoryExtensionLoader + Send>,
    ) -> Result<(), StdError> {
        extension::INSTANCE
            .add_invoker_directory_extension_loader(loader)
            .await
    }

    pub async fn remove_protocol_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_protocol_extension_loader(name)
            .await
    }

    pub async fn remove_registry_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_registry_extension_loader(name)
            .await
    }

    pub async fn remove_cluster_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_cluster_extension_loader(name)
            .await
    }

    pub async fn remove_load_balance_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_load_balance_extension_loader(name)
            .await
    }

    pub async fn remove_router_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_router_extension_loader(name)
            .await
    }

    pub async fn remove_invoker_directory_extension_loader(name: &str) -> Result<(), StdError> {
        extension::INSTANCE
            .remove_invoker_directory_extension_loader(name)
            .await
    }

    pub async fn load_protocol_extension(url: Url) -> Result<Box<dyn Protocol + Send>, StdError> {
        extension::INSTANCE.load_protocol_extension(url).await
    }

    pub async fn load_registry_extension(
        url: Url,
    ) -> Result<Box<dyn registry_extension::Registry + Send>, StdError> {
        extension::INSTANCE.load_registry_extension(url).await
    }

    pub async fn load_cluster_extension(
        url: Url,
    ) -> Result<Box<dyn cluster_extension::Cluster + Send>, StdError> {
        extension::INSTANCE.load_cluster_extension(url).await
    }

    pub async fn load_load_balance_extension(
        url: Url,
    ) -> Result<Box<dyn load_balance_extension::LoadBalance + Send>, StdError> {
        extension::INSTANCE.load_load_balance_extension(url).await
    }

    pub async fn load_router_extension(
        url: Url,
    ) -> Result<Box<dyn router_extension::Router + Send>, StdError> {
        extension::INSTANCE.load_router_extension(url).await
    }

    pub async fn load_invoker_directory_extension(
        url: Url,
    ) -> Result<Box<dyn directory_extension::InvokerDirectory + Send>, StdError> {
        extension::INSTANCE
            .load_invoker_directory_extension(url)
            .await
    }
}

impl DubboBoot {
    pub fn new() -> Self {
        Self {
            reference_configs: Vec::new(),
            registry_configs: Vec::new(),
        }
    }

    pub fn add_reference_config(&mut self, reference_config: ReferenceConfig) {
        self.reference_configs.push(reference_config);
    }

    pub fn add_registry_config(&mut self, registry_config: RegistryConfig) {
        self.registry_configs.push(registry_config);
    }

    pub async fn start(mut self) -> Dubbo {
        let reference = self.reference().await;

        Dubbo::new(reference)
    }

    pub async fn reference(&mut self) -> HashMap<String, CloneableInvoker> {
        let mut invoker_cache: HashMap<String, CloneableInvoker> = HashMap::new();

        for reference_config in self.reference_configs.iter() {
            let reference_url = reference_config.url().clone();
            let rust_type_name = reference_url.query::<RustTypeName>().unwrap();

            let mut invokers = Vec::new();

            for registry_configs in self.registry_configs.iter() {
                let registry_url = registry_configs.url().clone();

                let mut extension_url = extension::build_extension_loader_url(registry_url.host().unwrap(), "registry", registry_url.short_url_without_query().as_str());
                extension_url.add_query_param(RegistryUrl::new(registry_url.clone()));

                let mut protocol_extension =
                    DubboBoot::load_protocol_extension(extension_url)
                        .await
                        .unwrap();
                    
                let invoker = protocol_extension.refer(reference_url.clone()).await.unwrap();

                invokers.push(invoker);
            }

            if invokers.len() > 1 {
                // todo static cluster config url 
                let mut cluster_extension =
                    DubboBoot::load_cluster_extension(reference_url.clone())
                        .await
                        .unwrap();
                let invoker = cluster_extension.join(invokers).await.unwrap();
                invokers = vec![invoker];
            }

            invoker_cache.insert(rust_type_name.value(), CloneableInvoker::new(invokers.pop().unwrap()));
        }

        invoker_cache
    }
}
