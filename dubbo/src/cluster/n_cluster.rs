use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::MakeService;
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::cluster_extension::Cluster;
use crate::extension::loadbalancer_extension::LoadBalancer;
use crate::{StdError, Url};
use crate::params::cluster_params::ClusterType;
use crate::params::registry_param::InterfaceName;
use crate::url::UrlParam;

pub struct MkClusterBuilder<N> {
    inner: N
}


impl<N> MkClusterBuilder<N> {
    pub fn layer() -> impl tower_layer::Layer<N, Service = Self> {
        tower_layer::layer_fn(|inner: N| {
            MkClusterBuilder {
                inner
            }
        })
    }
}

impl<N, M> Service<DubboConfig> for MkClusterBuilder<N>
where
    N: MakeService<DubboConfig, M>,
    M: MakeService<Url, Box<dyn LoadBalancer + Send + 'static>>
{
    type Response = ClusterBuilder<M>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: DubboConfig) -> Self::Future {
        let cluster_type = req.cluster();

        let mk_load_balancer_builder = self.inner.make_service(req);

        let fut = async move {
            let load_balancer_builder = mk_load_balancer_builder.await?;
            let cluster_builder = ClusterBuilder::new(cluster_type, load_balancer_builder);
            Ok(cluster_builder)
        };

        Box::pin(fut)
    }
}


pub struct ClusterBuilder<N> {
    default_cluster_type: String,
    inner: N,
    cache: HashMap<String, Box<dyn Cluster + Send + 'static>>
}


impl<N>  ClusterBuilder<N> {

    pub fn new(default_cluster_type: String, inner: N) -> Self {
        ClusterBuilder {
            default_cluster_type,
            inner,
            cache: HashMap::new()
        }
    }
}

impl<N> Service<Url> for ClusterBuilder<N>
where
    N: MakeService<Url, Box<dyn LoadBalancer + Send + 'static>>
{
    type Response = Box<dyn Cluster + Send + 'static>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let cluster_type = req.query::<ClusterType>();
        let interface_name = req.query::<InterfaceName>();
        let Some(interface_name) = interface_name else {
          return Box::pin(async { Err(StdError::new("interface name is required")) } )
        };
        let interface_name = interface_name.value();
        let cached_cluster = self.cache.get(&interface_name).map(|cluster| {
            let cluster = Cluster::clone(cluster.as_ref());
            Box::pin(async move {
                Ok(cluster)
            })
        });

        match cached_cluster {
            Some(fut) => fut,
            None => {
                let fut = self.inner.make_service(req);
                let default_cluster_type = self.default_cluster_type.clone();
                let fut = async move {
                    let load_balancer = fut.await?;
                    let cluster = Cluster::new(default_cluster_type, load_balancer);
                    Ok(cluster)
                };
                Box::pin(fut)
            }
        }

    }
}
