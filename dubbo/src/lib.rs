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

use invoker::Invoker;

use crate::invoker::RpcInvocation;


pub mod cluster;
pub mod directory;
pub mod filter;
pub mod invoker;
pub mod loadbalancer;
pub mod protocol;
pub mod registry;
pub mod route;
pub mod extension;
pub mod config;
pub mod framework;

pub type StdError = Box<dyn std::error::Error + Send + Sync>;


#[async_trait::async_trait]
pub trait BackendService {
    
    async fn say_hello(&mut self, name: String) -> String;

    fn dubbo_service_metadata() 
    where
        Self: Sized,
    {

        let interface_name = "BackendService";



    }

    fn build_dubbo_service<T>(invoker: T) -> impl BackendService + Clone + Send
    where
        Self: Sized + Send,
        T: Invoker + Clone + Send + 'static
    {
        
        #[derive(Clone)]
        struct BackendServiceProxy<T> {
            invoker: T,
        }

        impl<T> BackendServiceProxy<T> {
            fn new(invoker: T) -> Self {
                Self {
                    invoker,
                }
            }
        }



        #[async_trait::async_trait]
        impl<T> BackendService for BackendServiceProxy<T>
        where
            T: Invoker + Clone + Send + 'static
        {

            async fn say_hello(&mut self, name: String) -> String {

                let service_name = "test".to_string();
                let interface_name = "BackendService".to_string();
                let method_name = "say_hello".to_string();

               // let rpc_inv = RpcInvocation::new();
                
                "".to_string()
            }
        }

        BackendServiceProxy::new(invoker)
    }
    
}