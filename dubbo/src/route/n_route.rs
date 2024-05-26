use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::route_extension::Router;
use crate::params::extension_params::{ExtensionName, ExtensionType, ExtensionUrl};
use crate::params::invoke_params::InvokeServiceName;
use crate::params::router_params::{RouterName, RouterServiceName, RouterType};
use crate::url::UrlParam;
use crate::{extension, StdError, Url};

#[derive(Clone)]
pub struct MkRouterBuilder;



impl Service<DubboConfig> for MkRouterBuilder {
    
    type Response = RouterBuilder;

    type Error = StdError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let fut = async move {
            let router_type = req.router();
            Ok(RouterBuilder {
                router_type
            })
        };
        Box::pin(fut)
    }
}


pub struct RouterBuilder {
    router_type: RouterType,
}


impl Service<Url> for RouterBuilder {
    type Response = Box<dyn Router + Send + Sync + 'static>;

    type Error = StdError;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let invoke_service_name = req.query::<InvokeServiceName>();
        let Some(invoke_service_name) = invoke_service_name else {
            return Box::pin(async move {
                return Err(RouterBuildError::new("service name mustn't be empty").into());
            });
        };

        let invoke_service_name = invoke_service_name.value();

        // build router url
        let router_name = RouterName::new(format!("{}-{}", invoke_service_name, self.router_type.as_str()));
        let router_service_name = RouterServiceName::new(invoke_service_name.as_str());
        let mut router_url = extension::route_extension::build_router_url(invoke_service_name.as_str());
        router_url.add_query_param(router_name);
        router_url.add_query_param(router_service_name);

        // build extension url
        let extension_name = ExtensionName::new(self.router_type.as_str());
        let extension_url = ExtensionUrl::new(router_url);
        let mut router_extension_url = extension::build_extension_url(ExtensionType::Router, extension_name);
        router_extension_url.add_query_param(extension_url);

        let fut = async move {
            let router = extension::EXTENSIONS.load_router(req).await?;
            Ok(router)
        };
        Box::pin(fut)
    }
}



#[derive(Debug, Error)]
#[error("Router build error: {0}")]
pub struct RouterBuildError(String);

impl RouterBuildError {
    pub fn new(msg: impl Into<String>) -> Self {
        RouterBuildError(msg.into())
    }

}