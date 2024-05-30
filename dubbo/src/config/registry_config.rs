use crate::common::url::Url;

#[derive(Debug, Clone)]
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

impl Default for RegistryConfig {
    fn default() -> Self {
        RegistryConfig {
            url: Url::empty(),
        }
    }
}