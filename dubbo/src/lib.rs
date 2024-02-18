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

use futures::{FutureExt, TryFutureExt};
use invoker::{cloneable_invoker::CloneableInvoker, Invoker};

use crate::{invoker::{Argument, RpcInvocation}, serialize::{Deserializable, SerdeJsonDeserialization, SerdeJsonSerialization}};

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
pub mod serialize;

pub type StdError = Box<dyn std::error::Error + Send + Sync>;


pub trait TestCall {
    
    fn say_hello(&self, name: String) -> String;
}


pub struct TestCallImpl {
    invoker: CloneableInvoker,
}


impl TestCall for TestCallImpl {
    
    fn say_hello(&self, name: String) -> String {
        let interface_name = "test";
        let method_name = "say_hello";

        
        let name_arg = Argument::new("name".to_string(), Box::new(SerdeJsonSerialization::new(name.clone())));


        let invocation = RpcInvocation::new(interface_name.to_string(), method_name.to_string(), vec![name_arg]);

        let mut invoker = self.invoker.clone();
        
        tokio::spawn(async move {
            let _ = invoker.ready().await;
            let rsp = invoker.invoke(invocation).await.unwrap();

            let body = rsp.into_body();
            
            let des: SerdeJsonDeserialization<String> = SerdeJsonDeserialization::<String>::new();
            let resp = des.deserialize(body);
        });

        format!("Hello, {}!", name)
    }
}
