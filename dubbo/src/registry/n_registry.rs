use tower::Service;

use crate::{config::dubbo_config::DubboConfig, extension::{self, registry_extension::Registry}, params::{extension_params::{ExtensionName, ExtensionType, ExtensionUrl}, registry_params::RegistryType}, url::UrlParam, StdError, Url};

pub struct MkRegistryBuilder;


impl Service<DubboConfig> for MkRegistryBuilder {
    
    type Response = RegistryBuilder;

    type Error = StdError;

    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let registry_url = req.registry();
        let fut = async move {
            Ok(RegistryBuilder {
                registry_url,
            })
        };
        Box::pin(fut)
    }
}

pub struct RegistryBuilder {
    registry_url: Url,
}



impl Service<Url> for RegistryBuilder {
    
    type Response = Box<dyn Registry + Send + Sync + 'static>;

    type Error = StdError;

    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: Url) -> Self::Future {
        let registry_url = self.registry_url.clone();
        let fut = async move {
            let registry_type = registry_url.query::<RegistryType>();
            let Some(registry_type) = registry_type else {
                return Err(StdError::from("load registry extension failed, registry type mustn't be empty"));
            };
            let registry_type = registry_type.value();

            // build extension registry
            let extension_name = ExtensionName::new(&registry_type);
            let extension_url = ExtensionUrl::new(registry_url);
            let mut registry_extension_url = extension::build_extension_url(ExtensionType::Registry, extension_name);
            registry_extension_url.add_query_param(extension_url);
        

            let registry = extension::EXTENSIONS.load_registry(registry_extension_url).await?;
            Ok(registry)
        };
        Box::pin(fut)
    }
}