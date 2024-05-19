use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::MakeService;
use tower_service::Service;
use crate::extension::invoker_extension::Invoker;
use crate::{StdError, Url};
use crate::extension::loadbalancer_extension::LoadBalancerChooser;

pub struct MkCluster<N> {
    inner: N,
    cache: HashMap<String, Arc<dyn LoadBalancerChooser>>
}


impl<N> MkCluster<N> {
    pub fn layer() -> impl tower_layer::Layer<N, Service = Self> {
        tower_layer::layer_fn(|inner: N| {
            MkCluster {
                inner,
                cache: HashMap::new()
            }
        })
    }
}

impl<N> Service<Url> for MkCluster<N> 
where
    N: MakeService<Arc<dyn LoadBalancerChooser>, Url>,
{
    type Response = Box<dyn Invoker>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Url) -> Self::Future {
        todo!()
    }
}


pub struct ClusterService {
    inner: Arc<dyn Invoker>
}

