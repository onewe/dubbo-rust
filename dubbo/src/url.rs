use std::str::FromStr;

#[derive(Debug, Clone)]
 pub(crate) struct Url {
    inner: url::Url,
}


impl Url {

    pub(crate) fn protocol(&self) -> &str {
        self.inner.scheme()
    }

    pub(crate) fn host(&self) -> Option<&str> {
        self.inner.host_str()
    }

    pub(crate) fn  username(&self) -> &str {
        self.inner.username()
    }

    pub(crate) fn password(&self) -> Option<&str> {
        self.inner.password()
    }

    pub(crate) fn port(&self) -> Option<u16> {
        self.inner.port_or_known_default()
    }

    pub(crate) fn path(&self) -> &str {
        self.inner.path()
    }

    pub(crate) fn query<T: UrlParam>(&self) -> Option<T> {
        self.inner.query_pairs()
            .find(|(k, _)| k == T::key())
            .map(|(_, v)| T::from(v.into_owned()))
    }

    pub(crate) fn set_protocol(&mut self, protocol: &str) {
        let _ = self.inner.set_scheme(protocol);
    }

    pub(crate) fn set_host(&mut self, host: &str) {
        let _ = self.inner.set_host(Some(host));
    }

    pub(crate) fn set_port(&mut self, port: u16) {
        let _ = self.inner.set_port(Some(port));
    }

    pub(crate) fn set_username(&mut self, username: &str) {
        let _ = self.inner.set_username(username);
    }

    pub(crate) fn set_password(&mut self, password: &str) {
        let _ = self.inner.set_password(Some(password));
    }

    pub(crate) fn set_path(&mut self, path: &str) {
        let _ = self.inner.set_path(path);
    }

    pub(crate) fn add_query_param<T: UrlParam>(&mut self, param: &T) {
        let mut pairs = self.inner.query_pairs_mut();
        pairs.append_pair(T::key(), param.value_to_string());
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



pub(crate) trait UrlParam: From<String> {

    type ParamType;

    fn key() -> &'static str;

    fn value(&self) -> Self::ParamType;

    fn value_to_string(&self) -> &str;
}

