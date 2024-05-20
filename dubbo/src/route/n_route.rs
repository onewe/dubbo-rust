use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::route_extension::Router;
use crate::{StdError, Url};

pub struct MkRouteBuilder;


impl Service<DubboConfig> for MkRouteBuilder
{
    type Response = RouteBuilder;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        todo!()
    }
}


pub struct RouteBuilder {
    default_route_type: String,
    cache: HashMap<String, Box<dyn Router + Send + 'static>>
}

impl Service<Url> for RouteBuilder {
    type Response = Box<dyn Router + Send + 'static>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future< Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Url) -> Self::Future {
        todo!()
    }
}