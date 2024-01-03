use crate::{url::Url, param::{Param, Registry}};

pub struct RegistryConfig {
    url: Url
}

impl RegistryConfig {

    pub fn new(url: Url) -> Self {
        Self {
            url
        }
    }

    pub fn protocol(&self) -> &str {
        self.url.protocol()
    }

    pub fn host(&self) -> Option<&str> {
        self.url.host()
    }

    pub fn username(&self) -> &str {
        self.url.username()
    }

    pub fn password(&self) -> Option<&str> {
        self.url.password()
    }

    pub fn port(&self) -> Option<u16> {
        self.url.port()
    }

    pub fn path(&self) -> &str {
        self.url.path()
    }

    pub fn query<T: Param>(&self) -> Option<T> {
        self.url.query()
    }

    pub fn set_protocol(&mut self, protocol: &str) {
        self.url.set_protocol(protocol);
    }


    pub fn set_host(&mut self, host: &str) {
        self.url.set_host(host);
    }

    pub fn set_port(&mut self, port: u16) {
        self.url.set_port(port);
    }

    pub fn set_username(&mut self, username: &str) {
        self.url.set_username(username);
    }

    pub fn set_password(&mut self, password: &str) {
        self.url.set_password(password);
    }

    pub fn set_path(&mut self, path: &str) {
        self.url.set_path(path);
    }

    pub fn add_query_param<T: Param>(&mut self, param: T) {
        self.url.add_query_param(param);
    }

    pub fn to_url(&self) -> Url {
        const REGISTRY_PROTOCOL: &str = "registry";

        let protocol = self.protocol();

        let mut url = self.url.clone();
        url.set_protocol(REGISTRY_PROTOCOL);

        match protocol.parse() {
            Ok(registry) => url.add_query_param::<Registry>(registry),
            Err(_) => {}
        }

        url
    }

}