/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;
use tower::MakeService;
use tower_service::Service;
use crate::common::url::params::cluster_params::{ClusterName, ClusterServiceName, ClusterType};
use crate::common::url::params::extension_params::{ExtensionName, ExtensionType, ExtensionUrl};
use crate::common::url::params::invoke_params::InvokeServiceName;
use crate::common::url::{Url, UrlParam};
use crate::config::dubbo_config::DubboConfig;
use crate::extension::loadbalancer_extension::LoadBalancerChooser;
use crate::extension::protocol_extension::Invoker;
use crate::{extension, StdError};


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
    N::Future: Send + 'static,
    N::Service: Service<Url, Response = Box<dyn LoadBalancerChooser + Send + Sync + 'static>, Error = StdError> + Send + 'static,
    <N::Service as Service<Url>>::Future: Send + 'static,
{
    type Response = ClusterBuilder<N::Service>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

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
    N::Future: Future<Output = Result<N::Response, N::Error>> + Send + 'static,
{
    type Response = Box<dyn Invoker + Send + Sync + 'static>;
    type Error = StdError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

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


impl<N: Clone> Clone for ClusterBuilder<N> {

    fn clone(&self) -> Self {
        Self { cluster_type: self.cluster_type.clone(), inner: self.inner.clone() }
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