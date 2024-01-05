use async_trait::async_trait;

use crate::{inv::Invoker, StdError};

use super::Directory;


pub struct StaticDirectory {
    cache_invokers: Vec<Box<dyn Invoker>>,
}

impl StaticDirectory {

    pub fn new(invokers: Vec<Box<dyn Invoker>>) -> Self {
        Self {
            cache_invokers: invokers,
        }
    }
}
