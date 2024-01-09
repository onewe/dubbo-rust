use async_trait::async_trait;

use crate::{inv::Invoker, StdError};

use super::Directory;
use crate::inv::cloneable_invoker::CloneableInvoker;


pub struct StaticDirectory {
    cache_invokers: Vec<CloneableInvoker>,
}

impl StaticDirectory {

    pub fn new(invokers: Vec<Box<dyn Invoker + Send + 'static>>) -> Self {
        let cache_invokers = invokers.into_iter().map(|invoker| {
            CloneableInvoker::new(invoker)
        }).collect::<Vec<CloneableInvoker>>();

       Self { cache_invokers }
    }
}


#[async_trait]
impl Directory for StaticDirectory {

    async fn list(&mut self) -> Result<Vec<CloneableInvoker>, StdError> {
      Ok(self.cache_invokers.clone())
    }
}