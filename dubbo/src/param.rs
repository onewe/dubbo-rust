use std::{borrow::Cow, str::FromStr};

pub trait Param: FromStr {

    type TargetType;

    fn name() -> &'static str;

    fn value(&self) -> Self::TargetType;

    fn as_str<'a>(&'a self) -> Cow<'a, str>;

}