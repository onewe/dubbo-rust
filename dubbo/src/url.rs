use std::str::FromStr;

 pub(crate) struct Url {
    inner: url::Url,
}


impl Url {

    pub(crate) fn protocol(&self) -> &str {
        self.inner.scheme()
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

    fn key() -> String;

    fn value(&self) -> Self::ParamType;

    fn value_to_string(&self) -> String;
}

