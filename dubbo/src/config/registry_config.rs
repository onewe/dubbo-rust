use crate::url::{Url, UrlParam};

pub(crate) struct RegistryConfig {
    url: Url
}

impl RegistryConfig {

    pub(crate) fn new(url: Url) -> Self {
        Self {
            url
        }
    }

    pub(crate) fn protocol(&self) -> &str {
        self.url.protocol()
    }

    pub(crate) fn host(&self) -> Option<&str> {
        self.url.host()
    }

    pub(crate) fn username(&self) -> &str {
        self.url.username()
    }

    pub(crate) fn password(&self) -> Option<&str> {
        self.url.password()
    }

    pub(crate) fn port(&self) -> Option<u16> {
        self.url.port()
    }

    pub(crate) fn path(&self) -> &str {
        self.url.path()
    }

    pub(crate) fn query<T: UrlParam>(&self) -> Option<T> {
        self.url.query()
    }

    pub(crate) fn set_protocol(&mut self, protocol: &str) {
        self.url.set_protocol(protocol);
    }


    pub(crate) fn set_host(&mut self, host: &str) {
        self.url.set_host(host);
    }

    pub(crate) fn set_port(&mut self, port: u16) {
        self.url.set_port(port);
    }

    pub(crate) fn set_username(&mut self, username: &str) {
        self.url.set_username(username);
    }

    pub(crate) fn set_password(&mut self, password: &str) {
        self.url.set_password(password);
    }

    pub(crate) fn set_path(&mut self, path: &str) {
        self.url.set_path(path);
    }

    pub(crate) fn add_query_param<T: UrlParam>(&mut self, param: &T) {
        self.url.add_query_param(param);
    }

}


pub(crate) struct UseAsConfigCenter(bool);

impl UrlParam for UseAsConfigCenter {

    fn key() -> &'static str {
        "useAsConfigCenter"
    }

    type ParamType = bool;

    fn value(&self) -> Self::ParamType {
        self.0
    }

    fn value_to_string(&self) -> &str {
        match self.0 {
            true => "true",
            false => "false"
        }
    }
}

impl From<String> for UseAsConfigCenter {

    fn from(s: String) -> Self {
        Self(s.parse::<bool>().unwrap())
    }
}

pub(crate) struct UseAsMetadataCenter(bool);

impl UrlParam for UseAsMetadataCenter {

    fn key() -> &'static str {
        "useAsMetadataCenter"
    }

    type ParamType = bool;

    fn value(&self) -> Self::ParamType {
        self.0
    }

    fn value_to_string(&self) -> &str {
        match self.0 {
            true => "true",
            false => "false"
        }
    }
}

impl From<String> for UseAsMetadataCenter {

    fn from(s: String) -> Self {
        Self(s.parse::<bool>().unwrap())
    }
}

pub(crate) struct EnableEmptyProtection(bool);

impl UrlParam for EnableEmptyProtection {

    fn key() -> &'static str {
        "enableEmptyProtection"
    }

    type ParamType = bool;

    fn value(&self) -> Self::ParamType {
        self.0
    }

    fn value_to_string(&self) -> &str {
        match self.0 {
            true => "true",
            false => "false"
        }
    }
}

impl From<String> for EnableEmptyProtection {

    fn from(s: String) -> Self {
        Self(s.parse::<bool>().unwrap())
    }
}