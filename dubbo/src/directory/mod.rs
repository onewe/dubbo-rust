use async_trait::async_trait;

use crate::{inv::{Invoker, cloneable_invoker::CloneableInvoker}, StdError};

mod static_directory;
mod dynamic_directory;


#[async_trait]
pub trait Directory {
    
    async fn list(&mut self) -> Result<Vec<CloneableInvoker>, StdError>;
}