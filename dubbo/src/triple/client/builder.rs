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

use std::sync::Arc;

use crate::cluster::MkClusterBuilder;
use crate::config::dubbo_config::DubboConfig;
use crate::directory::MkInvokerDirectoryBuilder;
use crate::loadbalancer::MkLoadBalancerBuilder;
use crate::registry::MkRegistryBuilder;
use crate::route::MkRouterBuilder;
use crate::{extension, utils::boxed_clone::BoxCloneService,};

use crate::{cluster, directory, loadbalancer, registry, route, Dubbo, Url};
use aws_smithy_http::body::SdkBody;
use tower::{Layer, MakeService, Service, ServiceBuilder};

pub type ClientBoxService =
    BoxCloneService<http::Request<SdkBody>, http::Response<crate::BoxBody>, crate::Error>;


#[derive(Default)]
pub struct ClientBuilder {
    pub timeout: Option<u64>,
    pub connector: &'static str,
    registry_extension_url: Option<Url>,
    pub direct: bool,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            timeout: None,
            connector: "",
            registry_extension_url: None,
            direct: false,
        }
    }

    pub fn from_static(host: &str) -> ClientBuilder {
        // let registry_extension_url = StaticRegistry::to_extension_url(vec![host.parse().unwrap()]);
        // Self {
        //     timeout: None,
        //     connector: "",
        //     registry_extension_url: Some(registry_extension_url),
        //     direct: true,
        // }

        todo!()
    }

    pub fn with_timeout(self, timeout: u64) -> Self {
        Self {
            timeout: Some(timeout),
            ..self
        }
    }

    pub fn with_registry(self, registry: Url) -> Self {
        todo!()
    }

    pub fn with_host(self, host: &'static str) -> Self {
        todo!()
    }

    pub fn with_connector(self, connector: &'static str) -> Self {
        Self { connector, ..self }
    }

    pub fn with_direct(self, direct: bool) -> Self {
        Self { direct, ..self }
    }

    pub async  fn build(mut self) {
        let registry = self
            .registry_extension_url
            .take()
            .expect("registry must not be empty");

        let mut builder = ServiceBuilder::new()
            .layer(cluster::MkClusterBuilder::layer())
            .layer(loadbalancer::MkLoadBalancerBuilder::layer(route::MkRouterBuilder))
            .layer(directory::MkInvokerDirectoryBuilder::layer())
            .service(registry::MkRegistryBuilder);


            // let mut r = route::MkRouterBuilder;
            // let b = r.make_service(DubboConfig).await.unwrap();
            // let c = b.call("".parse().unwrap()).await.unwrap();

        let dubbo_config = DubboConfig;

        let s = builder.make_service(dubbo_config).await.unwrap();


        // let mk_service = builder.make_service(dubbo_config);

        // let mk_service = ServiceBuilder::new()
        //     .layer(NewCluster::layer())
        //     .layer(NewLoadBalancer::layer())
        //     .layer(NewRoutes::layer())
        //     .layer(NewCachedDirectory::layer())
        //     .service(MkRegistryService::new(registry));

        // Arc::new(mk_service)
    }
}
