use crate::url::Url;

use super::MethodConfig;

#[derive(Default, Debug)]
pub(crate) struct ReferenceConfig {

    service: String,

    interface_name: String,

    methods: Vec<MethodConfig>,

    url: Option<Url>
}

impl ReferenceConfig {

    pub(crate) fn new(service: String, interface_name: String, methods: Vec<MethodConfig>, url: Option<Url>) -> Self {
        Self {
            service,
            interface_name,
            methods,
            url,
        }
    }

    pub(crate) fn service(&self) -> &str {
        &self.service
    }

    pub(crate) fn interface_name(&self) -> &str {
        &self.interface_name
    }

    pub(crate) fn methods(&self) -> &Vec<MethodConfig> {
        &self.methods
    }

    pub(crate) fn url(&self) -> Option<&Url> {
        self.url.as_ref()
    }


    pub(crate) fn set_service(&mut self, service: String) {
        self.service = service;
    }

    pub(crate) fn set_interface_name(&mut self, interface_name: String) {
        self.interface_name = interface_name;
    }

    pub(crate) fn set_methods(&mut self, methods: Vec<MethodConfig>) {
        self.methods = methods;
    }

    pub(crate) fn add_method(&mut self, method: MethodConfig) {
        self.methods.push(method);
    }

    pub(crate) fn set_url(&mut self, url: Url) {
        self.url = Some(url);
    }


}