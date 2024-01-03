use std::str::FromStr;

use crate::param::Param;

#[derive(Debug, Clone)]
 pub struct Url {
    inner: url::Url,
}


impl Url {

    pub fn empty() -> Self {
        "empty://localhost".parse().unwrap()
    }

    pub fn protocol(&self) -> &str {
        self.inner.scheme()
    }

    pub fn host(&self) -> Option<&str> {
        self.inner.host_str()
    }

    pub fn  username(&self) -> &str {
        self.inner.username()
    }

    pub fn password(&self) -> Option<&str> {
        self.inner.password()
    }

    pub fn port(&self) -> Option<u16> {
        self.inner.port_or_known_default()
    }

    pub fn path(&self) -> &str {
        self.inner.path()
    }

    pub fn query<T: Param>(&self) -> Option<T> {
        self.inner.query_pairs()
            .find(|(k, _)| k == T::name())
            .map(|(_, v)| T::from_str(&v).ok())
            .flatten()
            
            
    }

    pub fn set_protocol(&mut self, protocol: &str) {
        let _ = self.inner.set_scheme(protocol);
    }

    pub fn set_host(&mut self, host: &str) {
        let _ = self.inner.set_host(Some(host));
    }

    pub fn set_port(&mut self, port: u16) {
        let _ = self.inner.set_port(Some(port));
    }

    pub fn set_username(&mut self, username: &str) {
        let _ = self.inner.set_username(username);
    }

    pub fn set_password(&mut self, password: &str) {
        let _ = self.inner.set_password(Some(password));
    }

    pub fn set_path(&mut self, path: &str) {
        let _ = self.inner.set_path(path);
    }

    pub fn add_query_param<T: Param>(&mut self, param: T) {
        let mut pairs = self.inner.query_pairs_mut();
        pairs.append_pair(T::name(), &param.as_str());
    }

}



impl FromStr for Url {

    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Url {
            inner: url::Url::parse(s)?,
        })
    }
}