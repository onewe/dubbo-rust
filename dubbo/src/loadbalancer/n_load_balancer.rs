use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures_core::ready;
use tower::MakeService;
use tower_layer::Layer;
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::invoker_directory_extension::InvokerList;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::extension::route_extension::Router;
use crate::{StdError, Url};

pub struct MkLoadBalancerBuilder<N, M> {
    mk_invoker_list_builder: N,
    mk_router_builder: M,
}


impl<N, M> MkLoadBalancerBuilder<N, M> {

    pub fn new(mk_invoker_list_builder: N, mk_router_builder: M) -> Self {
        MkLoadBalancerBuilder {
            mk_invoker_list_builder,
            mk_router_builder,
        }
    }
}

impl<N, M> MkLoadBalancerBuilder<N, M> {
    pub fn layer(mk_router_builder: M) -> impl Layer<N, Service = Self> {
        tower_layer::layer_fn(move |mk_invoker_list_builder: N| {
            MkLoadBalancerBuilder {
                mk_invoker_list_builder,
                mk_router_builder,
            }
        })
    }
}


impl<N, M, J, K> Service<DubboConfig> for MkLoadBalancerBuilder<N, M>
where
    N: MakeService<DubboConfig, J>,
    J: MakeService<Url, Box<dyn InvokerList + Send + 'static>>,
    M: MakeService<DubboConfig, K>,
    K: MakeService<Url, Box<dyn Router + Send + 'static>>,
{
    type Response = LoadBalancerBuilder<J, K>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let _ = ready!(self.mk_router_builder.poll_ready(cx))?;
        self.mk_invoker_list_builder.poll_ready(cx)
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let invoker_list_builder = self.mk_invoker_list_builder.make_service(req.clone());
        let router_builder = self.mk_router_builder.make_service(req);
        let fut = async move {
            let invoker_list = invoker_list_builder.await?;
            let router = router_builder.await?;
            let load_balancer = LoadBalancerBuilder::new(invoker_list, router);
            Ok(load_balancer)
        };

        Box::pin(fut)
    }
}


pub struct LoadBalancerBuilder<N, M> {
    invoker_list_builder: N,
    router_builder: M,
    cache: HashMap<String, Box<dyn LoadBalancerChooser>>
}


impl<N, M> Service<Url> for LoadBalancerBuilder<N, M>
where
    N: MakeService<Url, Box<dyn InvokerList + Send + 'static>>,
    M: MakeService<Url, Box<dyn Router + Send + 'static>>,
{
    type Response = Box<dyn LoadBalancerChooser>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let _ = ready!(self.router_builder.poll_ready(cx))?;
        self.invoker_list_builder.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let invoker_list_builder = self.invoker_list_builder.make_service(req.clone());
        let router_builder = self.router_builder.make_service(req);
        let fut = async move {
            let invoker_list = invoker_list_builder.await?;
            let router = router_builder.await?;
            Ok(router)
        };

        Box::pin(fut)
    }
}