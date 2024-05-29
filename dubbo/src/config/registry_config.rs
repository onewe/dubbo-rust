use crate::common::url::Url;

pub struct RegistryConfig {
    url: Url,
}

impl RegistryConfig {
    pub fn new(url: Url) -> Self {
        RegistryConfig {
            url,
        }
    }
}


impl RegistryConfig {

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}