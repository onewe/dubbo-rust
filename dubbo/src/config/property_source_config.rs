use crate::common::url::Url;

pub struct PropertySourceConfig {
    url: Url,
}


impl PropertySourceConfig {
    pub fn new(url: Url) -> Self {
        PropertySourceConfig {
            url,
        }
    }
}

impl PropertySourceConfig {

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}