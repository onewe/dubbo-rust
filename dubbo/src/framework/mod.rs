use std::collections::HashMap;

use crate::invoker::{cloneable_invoker::CloneableInvoker, Invoker};

pub mod boot;


pub struct Dubbo {

    cache: HashMap<String, CloneableInvoker>

}

impl Dubbo {


    pub fn new(cache: HashMap<String, CloneableInvoker>) -> Self {
        Self {
            cache,
        }
    }

    pub fn add_invoker(&mut self, type_name: String, invoker: Box<dyn Invoker + Send>) {
        self.cache.insert(type_name, CloneableInvoker::new(invoker));
    }

    pub fn get_service<S>(&self) -> Option<S>
    where
        S: DubboService
    {
        let type_name = std::any::type_name::<S>();
        self.cache.get(type_name).map(|invoker| {
            let invoker = invoker.clone();
            S::build(invoker)
        })
    }
}




pub trait DubboService: Clone {
    
    fn service_metadata() -> ServiceMetadata;

    fn build<Inv: Invoker + Clone>(invoker: Inv) -> Self;
}


pub struct ServiceMetadata {
    
    pub interface_name: String,

    pub method_names: Vec<String>
}


