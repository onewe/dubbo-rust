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

use cluster_config::ClusterConfig;
use invoker_directory_config::InvokerDirectoryConfig;
use load_balancer_config::LoadBalancerConfig;
use property_source_config::PropertySourceConfig;
use registry_config::RegistryConfig;
use route_config::RouteConfig;


pub mod cluster_config;
pub mod invoker_directory_config;
pub mod load_balancer_config;
pub mod property_source_config;
pub mod registry_config;
pub mod route_config;
pub mod dubbo_config;


#[derive(Clone, Default)]
pub struct DubboConfig {
    pub(crate) cluster_config: ClusterConfig,
    pub(crate) invoker_directory_config: InvokerDirectoryConfig,
    pub(crate) load_balancer_config: LoadBalancerConfig,
    pub(crate) property_source_config: PropertySourceConfig,
    pub(crate) registry_config: RegistryConfig,
    pub(crate) route_config: RouteConfig,
}



impl DubboConfig {

    
    pub fn cluster_config(self, cluster_config: ClusterConfig) -> Self {
        DubboConfig {
            cluster_config,
            ..self
        }
    }

    pub fn invoker_directory_config(self, invoker_directory_config: InvokerDirectoryConfig) -> Self {
        DubboConfig {
            invoker_directory_config,
            ..self
        }
    }

    pub fn load_balancer_config(self, load_balancer_config: LoadBalancerConfig) -> Self {
        DubboConfig {
            load_balancer_config,
            ..self
        }
    }

    pub fn property_source_config(self, property_source_config: PropertySourceConfig) -> Self {
        DubboConfig {
            property_source_config,
            ..self
        }
    }

    pub fn registry_config(self, registry_config: RegistryConfig) -> Self {
        DubboConfig {
            registry_config,
            ..self
        }
    }

    pub fn route_config(self, route_config: RouteConfig) -> Self {
        DubboConfig {
            route_config,
            ..self
        }
    }
}