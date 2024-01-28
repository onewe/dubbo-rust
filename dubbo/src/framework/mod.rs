use std::{any::TypeId, collections::HashMap};

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

    pub fn add_invoker(&mut self, type_id: String, invoker: Box<dyn Invoker + Send>) {
        self.cache.insert(type_id, CloneableInvoker::new(invoker));
    }

    pub fn get_service<S>(&self) -> Option<S>
    where
        S: DubboService + 'static
    {
        let type_id = S::type_id();
        self.cache.get(&type_id).map(|invoker| {
            let invoker = invoker.clone();
            S::build(invoker)
        })
    }
}




pub trait DubboService: Clone {
    
    fn service_metadata() -> ServiceMetadata;

    fn build<Inv: Invoker + Clone>(invoker: Inv) -> Self;

    fn type_id() -> String 
    where
        Self: 'static
    {
        
        let type_id = TypeId::of::<Self>();
        format!("{:?}", type_id)
    }
}


pub struct ServiceMetadata {
    
    pub service_name: String,

    pub method_names: Vec<String>
}


