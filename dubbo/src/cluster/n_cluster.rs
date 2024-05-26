use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;
use tower::MakeService;
use tower_service::Service;
use crate::config::dubbo_config::DubboConfig;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::extension::protocol_extension::Invoker;
use crate::params::extension_params::{ExtensionName, ExtensionType, ExtensionUrl};
use crate::params::invoke_params::InvokeServiceName;
use crate::{extension, StdError, Url};
use crate::params::cluster_params::{ClusterName, ClusterServiceName, ClusterType};
use crate::url::UrlParam;

pub struct MkClusterBuilder<N> {
    inner: N,
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

impl<N> Service<DubboConfig> for MkClusterBuilder<N>
where
    N: MakeService<DubboConfig, Url, MakeError = StdError>,
    <N as MakeService<DubboConfig, Url>>::Future: Send + 'static,
    <N as MakeService<DubboConfig, Url>>::Service: Service<Url, Response = Box<dyn LoadBalancerChooser + Send + Sync + 'static>, Error = StdError>,
{
    type Response = ClusterBuilder<<N as MakeService<DubboConfig, Url>>::Service>;
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
    cluster_type: ClusterType,
    inner: N,
}


impl<N>  ClusterBuilder<N> {

    pub fn new(cluster_type: ClusterType, inner: N) -> Self {
        ClusterBuilder {
            cluster_type,
            inner,
        }
    }
}

impl<N> Service<Url> for ClusterBuilder<N>
where
    N: Service<Url, Response = Box<dyn LoadBalancerChooser + Send + Sync + 'static>, Error = StdError>,
    <N as Service<Url>>::Future: Future<Output = Result<N::Response, N::Error>> + Send + 'static,
{
    type Response = Box<dyn Invoker + Send + Sync + 'static>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Url) -> Self::Future {
        let invoke_service_name = req.query::<InvokeServiceName>();
        let Some(invoke_service_name) = invoke_service_name else {
          return Box::pin(async { Err(ClusterBuilderError::new("service name is required").into()) } )
        };
        let invoke_service_name = invoke_service_name.value();
        let fut = self.inner.call(req);
        let cluster_type = self.cluster_type.clone();

        // build cluster url
        let mut cluster_url = extension::cluster_extension::build_cluster_url();
        // cluster name
        let cluster_name = ClusterName::new(format!("{}-{}",invoke_service_name, cluster_type.as_str()));
        // cluster service name
        let cluster_service_name = ClusterServiceName::new(&invoke_service_name);
        cluster_url.add_query_param(cluster_name);
        cluster_url.add_query_param(cluster_service_name);
        cluster_url.add_query_param(cluster_type);
        



        // build extension url
        let cluster_extension_name = ExtensionName::new(format!("{}-{}",invoke_service_name, self.cluster_type.as_str()));
        let extension_url = ExtensionUrl::new(cluster_url);
        let mut cluster_extension_url = extension::build_extension_url(ExtensionType::Cluster, cluster_extension_name);
        cluster_extension_url.add_query_param(extension_url);



        let fut = async move {
            let mut load_balancer_chooser = fut.await?;
            let mut cluster = extension::EXTENSIONS.load_cluster(cluster_extension_url).await?;

            let _ = load_balancer_chooser.ready().await?;
            let _ = cluster.ready().await?;

            let invoker = cluster.join(load_balancer_chooser).await?;
            Ok(invoker)
        };
        Box::pin(fut)

    }
}


#[derive(Debug, Error)]
#[error("build cluster occur an error: {0}")]
pub struct ClusterBuilderError(String);


impl ClusterBuilderError {
    pub fn new(msg: impl Into<String>) -> Self {
        ClusterBuilderError(msg.into())
    }
}