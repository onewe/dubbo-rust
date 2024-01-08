use crate::param::ReferenceUrl;

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
mod extension;
mod url;
mod config;
mod inv;
mod param; 
mod directory;

pub type StdError = Box<dyn std::error::Error + Send + Sync>;

pub struct DubboBootstrap {
    extension_directory: extension::ExtensionDirectory,
    application_configs: Vec<config::ApplicationConfig>,
    registry_configs: Vec<config::RegistryConfig>,
    reference_configs: Vec<config::ReferenceConfig>,
}

impl DubboBootstrap {

    pub fn new() -> Self {
        DubboBootstrap {
            application_configs: Vec::new(),
            registry_configs: Vec::new(),
            reference_configs: Vec::new(),
            extension_directory: extension::ExtensionDirectory::new(),
        }
    }

    pub fn add_protocol_extension_loader(&mut self, loader: Box<dyn extension::ProtocolExtensionLoader>) {
        self.extension_directory.add_protocol_extension_loader(loader);
    }

    pub fn add_registry_extension_loader(&mut self, loader: Box<dyn extension::RegistryExtensionLoader>) {
        self.extension_directory.add_registry_extension_loader(loader);
    }

    pub fn add_cluster_extension_loader(&mut self, loader: Box<dyn extension::ClusterExtensionLoader>) {
        self.extension_directory.add_cluster_extension_loader(loader);
    }

    pub fn add_load_balance_extension_loader(&mut self, loader: Box<dyn extension::LoadBalanceExtensionLoader>) {
        self.extension_directory.add_load_balance_extension_loader(loader);
    }

    pub fn add_router_extension_loader(&mut self, loader: Box<dyn extension::RouterExtensionLoader>) {
        self.extension_directory.add_router_extension_loader(loader);
    }

    pub fn add_application_config(&mut self, config: config::ApplicationConfig) {
        self.application_configs.push(config);
    }

    pub fn add_registry_config(&mut self, config: config::RegistryConfig) {
        self.registry_configs.push(config);
    }

    pub fn add_reference_config(&mut self, config: config::ReferenceConfig) {
        self.reference_configs.push(config);
    }

    pub async fn start(&mut self) {

    }

    async fn refer_services(&mut self) -> Result<(), StdError> {

        let mut urls = Vec::new();
        
        for reference_config in self.reference_configs.iter() {
            match reference_config.direct_url() {
                Some(direct_url) => {
                    urls.push(direct_url.clone())
                },
                None => {
                    let reference_url = reference_config.to_url();
                    for registry_config in  self.registry_configs.iter() {
                        let mut registry_url = registry_config.to_url();
                        registry_url.add_query_param(ReferenceUrl::new(reference_url.clone()));
                        urls.push(registry_url)
                    }
                }
            }
        }

        for url in urls.iter() {
            
            let protocol_extension_loader = self.extension_directory.find_protocol_extension_loader(url).await;
            match protocol_extension_loader {
                None => {},
                Some(protocol_extension_loader) => {
                    let protocol_extension = protocol_extension_loader.load(url).await?;
                    let invoker = protocol_extension.refer(url).await?;
                    
                }
            }

        }


        todo!()
    }
}


