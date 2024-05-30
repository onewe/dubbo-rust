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
use crate::{common::url::{Url, UrlParam}, StdError};
use std::{borrow::Cow, str::FromStr};



pub struct RegistryType(String);

impl RegistryType {
    pub fn new(ty: impl Into<String>) -> Self {
        Self(ty.into())
    }
}


impl UrlParam for RegistryType {
    
    type TargetType = String;

    fn name() -> &'static str {
        "registry-type"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}

impl FromStr for RegistryType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}


pub struct RegistryServiceName(String);

impl RegistryServiceName {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self(service_name.into())
    }
}

impl UrlParam for RegistryServiceName {
    
    type TargetType = String;

    fn name() -> &'static str {
        "registry-service-name"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str().into()
    }
}


impl FromStr for RegistryServiceName {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}


pub struct RegistryUrl(Url);

impl RegistryUrl {
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}

impl UrlParam for RegistryUrl {
    
    type TargetType = Url;

    fn name() -> &'static str {
        "registry-url"
    }

    fn value(&self) -> Self::TargetType {
        self.0.clone()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.to_string().into()
    }
}

impl FromStr for RegistryUrl {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.parse()?))
    }
}