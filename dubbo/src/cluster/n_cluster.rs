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
use anyhow::anyhow;

pub struct MkClusterBuilder<N, M> {
    inner: N,
    _marker: std::marker::PhantomData<M>
}


impl<N, M> MkClusterBuilder<N, M> {
    pub fn layer() -> impl tower_layer::Layer<N, Service = Self> {
        tower_layer::layer_fn(|inner: N| {
            MkClusterBuilder {
                inner,
                _marker: std::marker::PhantomData
            }
        })
    }
}

impl<N, M> Service<DubboConfig> for MkClusterBuilder<N, M>
where
    N: MakeService<DubboConfig, Url, Service = M, MakeError = StdError>,
    <N as MakeService<DubboConfig, Url>>::Future: Future<Output = Result<N::Service, N::MakeError>> + Send + 'static,
    M: Service<Url, Response = Box<dyn LoadBalancer + Send>, Error = StdError>
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
    cache: HashMap<String, Box<dyn Cluster + Send>>
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
    N: Service<Url, Response = Box<dyn LoadBalancer + Send>, Error = StdError>
{
    type Response = Box<dyn Cluster + Send>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let cluster_type = req.query::<ClusterType>();
        let interface_name = req.query::<InterfaceName>();
        let Some(interface_name) = interface_name else {
          return Box::pin(async { Err(anyhow!("interface name is required").into()) } )
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
                let fut = self.inner.call(req);

                let cluster_type = if let Some(cluster_type) = cluster_type {
                    cluster_type.value()
                } else {
                    self.default_cluster_type.clone()
                };

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
