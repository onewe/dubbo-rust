use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use tokio::sync::watch;
use tracing::{warn, debug};

use crate::{inv::cloneable_invoker::CloneableInvoker, StdError, url::Url, extension::{self, protocol_extension::Protocol}};

use super::Directory;

pub(crate) struct DynamicDirectory {

    service_watcher: watch::Receiver<HashSet<String>>,

    invoker_cache: HashMap<String, CloneableInvoker>

}


impl DynamicDirectory {

    pub fn new(service_watcher: watch::Receiver<HashSet<String>>) -> Self {
        Self {
            service_watcher,
            invoker_cache: HashMap::new(),
        }
    }

    fn change(&mut self, new_service_urls: HashSet<String>) -> (HashMap<String, String>, HashSet<String>){
        let old_service_list: HashMap<String, String> = self.invoker_cache.keys().map(|url| {
            let url = url.parse::<Url>();
            match url {
                Err(_) => {
                    None
                },
                Ok(url) => {
                    Some((url.short_url_without_query(), url.into()))
                }
            }
        }).flatten().collect();

        let new_service_list: HashMap<String, String> = new_service_urls.into_iter().map(|url| {
            let url = url.parse::<Url>();
            match url {
                Err(_) => {
                    None
                },
                Ok(url) => {
                    Some((url.short_url_without_query(), url.into()))
                }
            }
        }).flatten().collect();


        let mut to_add = HashMap::new();
        let mut to_remove = HashSet::new();

        for (key, new_value) in new_service_list.iter() {
            if !old_service_list.contains_key(key) {
                to_add.insert(key.clone(), new_value.clone());
            }
        }

        for (key, old_value) in old_service_list.iter() {

            match new_service_list.get(key) {
                None => {
                    to_remove.insert(key.clone());
                },
                Some(new_value) => {
                    if new_value != old_value {
                        to_remove.insert(key.clone());
                        to_add.insert(key.clone(), new_value.clone());
                    }
                }
            }
        }

        
        (to_add, to_remove)
    }
}

#[async_trait]
impl Directory for DynamicDirectory {


    async fn list(&mut self) -> Result<Vec<CloneableInvoker>, StdError> {

        match self.service_watcher.has_changed() {
            Err(e) => {
                warn!("service subscribe may be closed, use cached invokers, error: {}", e);
                let invokers: Vec<_> = self.invoker_cache.values().map(|invoker| invoker.clone()).collect();
                Ok(invokers)
            },
            Ok(true) => {
                debug!("service list changed, update invoker cache.");
                let invokers = self.service_watcher.borrow_and_update().clone();
                let (to_add, to_remove) = self.change(invokers);
                for key in to_remove.iter() {
                    self.invoker_cache.remove(key);
                }

                for (key, value) in to_add {
                   
                    let invoker_url = value.parse::<Url>();
                    match invoker_url {
                        Err(e) => {
                            warn!("parse invoker url failed, url: {}, error: {}", value, e);
                            continue;
                        },
                        Ok(invoker_url) => {
                            let mut extension = extension::load_protocol_extension(invoker_url.clone()).await?;
                            let invoker = extension.refer(invoker_url).await?;
                            let invoker = CloneableInvoker::new(invoker);
                            self.invoker_cache.insert(key, invoker);
                        }
                    }
                }

                let invokers: Vec<_> = self.invoker_cache.values().map(|invoker| invoker.clone()).collect();
                Ok(invokers)
            },
            Ok(false) => {
                debug!("service list not changed, use cached invokers.");
                let invokers: Vec<_> = self.invoker_cache.values().map(|invoker| invoker.clone()).collect();
                Ok(invokers)
            }

        }
    }
}