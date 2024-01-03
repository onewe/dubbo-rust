use crate::{url::Url, param::{Interface, Side, Revision, Methods, RegisterIp}};

use super::MethodConfig;

#[derive(Default, Debug)]
pub struct ReferenceConfig {

    service: String,

    interface_name: String,

    methods: Vec<MethodConfig>,

    version: String,

    url: Option<Url>
}

impl ReferenceConfig {

    pub fn new(service: String, interface_name: String, methods: Vec<MethodConfig>, version: String, url: Option<Url>) -> Self {
        Self {
            service,
            interface_name,
            methods,
            version,
            url,
        }
    }

    pub fn service(&self) -> &str {
        &self.service
    }

    pub fn interface_name(&self) -> &str {
        &self.interface_name
    }

    pub fn methods(&self) -> &Vec<MethodConfig> {
        &self.methods
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }


    pub fn set_service(&mut self, service: String) {
        self.service = service;
    }

    pub fn set_interface_name(&mut self, interface_name: String) {
        self.interface_name = interface_name;
    }

    pub fn set_methods(&mut self, methods: Vec<MethodConfig>) {
        self.methods = methods;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn add_method(&mut self, method: MethodConfig) {
        self.methods.push(method);
    }

    pub fn set_url(&mut self, url: Url) {
        self.url = Some(url);
    }


    pub fn to_url(&self) -> Url {

        const DEFAULT_PROTOCOL: &str = "dubbo-reference";
        const DEFAULT_HOST: &str = "localhost";

        let mut url = Url::empty();

        url.set_protocol(DEFAULT_PROTOCOL);
        url.set_host(DEFAULT_HOST);

        match self.interface_name.parse() {
            Ok(interface) => url.add_query_param::<Interface>(interface),
            Err(_) => {}
        }

        url.add_query_param(Side::Consumer);


        match self.version.parse() {
            Ok(revision) => url.add_query_param::<Revision>(revision),
            Err(_) => {}
        }

        if self.methods.is_empty() {
            url.add_query_param(Methods::Any)
        } else {
            url.add_query_param(Methods::Methods(self.methods.iter().map(|m| m.name().to_owned()).collect()))
        }

        url.add_query_param(RegisterIp::LocalHost);

        url
    }

}
