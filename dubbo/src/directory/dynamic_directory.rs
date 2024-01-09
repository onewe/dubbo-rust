use std::{collections::{HashMap, HashSet}, pin::Pin, future::Pending, task::Poll};

use async_trait::async_trait;
use futures::{Stream, future::poll_fn};
use tokio_stream::StreamExt;

use crate::{inv::cloneable_invoker::CloneableInvoker, StdError};

use super::Directory;

pub(crate) struct DynamicDirectory {

    change_stream: Box<dyn Stream<Item = HashSet<String>> + Send + Unpin>,

    invoker_cache: HashMap<String, CloneableInvoker>

}


impl DynamicDirectory {

    pub fn new(change_stream: Box<dyn Stream<Item = HashSet<String>> + Send + Unpin>) -> Self {
        Self {
            change_stream,
            invoker_cache: HashMap::new(),
        }
    }

    fn change(&mut self, change_list: HashSet<String>) {
        let old_service_list: HashSet<_> = self.invoker_cache.keys().collect();

        
    }
}

#[async_trait]
impl Directory for DynamicDirectory {


    async fn list(&mut self) -> Result<Vec<CloneableInvoker>, StdError> {

        let mut the_last_change_list = HashSet::new();
        loop {
            let change_list = poll_fn(|cx| {
                let change_stream = Pin::new(self.change_stream.as_mut());
                match change_stream.poll_next(cx) {
                    Poll::Pending => {
                        if self.invoker_cache.is_empty() {
                            Poll::Pending
                        } else {
                            Poll::Ready(HashSet::new())
                        }
                    },
                    Poll::Ready(Some(list)) => {
                        Poll::Ready(list)
                    },
                    Poll::Ready(None) => {
                        Poll::Ready(HashSet::new())
                    },
                }
            }).await;
            if change_list.is_empty() {
                break;
            }
            the_last_change_list = change_list;
        }
        
        // let next = self.change_stream.try_next().await;

        todo!()
    }
}