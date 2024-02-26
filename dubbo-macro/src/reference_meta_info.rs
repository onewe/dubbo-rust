use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex, OnceLock};
use syn::Token;
use crate::common::{Deserialization, Serialization};


const REFERENCE_META_INFO_G_SEQ: AtomicUsize = AtomicUsize::new(0);

static REFERENCE_META_INFO: OnceLock<Arc<Mutex<Vec<ReferenceMetaInfo>>>> = OnceLock::new();

#[derive(Clone)]
pub struct ReferenceMetaInfo {
    interface: Option<String>,
    serialization: Serialization,
    deserialization: Deserialization,
    idx: usize,
}

impl ReferenceMetaInfo {
    pub fn interface(&self) -> Option<String> {
        self.interface.clone()
    }
    
    pub fn serialization(&self) -> Serialization {
        self.serialization.clone()
    }
    
    pub fn deserialization(&self) -> Deserialization {
        self.deserialization.clone()
    }
    
    pub fn idx(&self) -> usize {
        self.idx
    }
}

impl ReferenceMetaInfo {
    pub fn get_from_global(idx: usize) -> Option<Self> {
        let meta_infos = REFERENCE_META_INFO.get_or_init(|| {
            let  v = Vec::new();
            Arc::new(Mutex::new(v))
        });
    
        let meta_infos = meta_infos.lock().expect("can not get reference meta info lock");
        meta_infos.get(idx).map(|x| x.clone())
    }
    
    pub fn get_from_global_by_reference_name(reference_name: &str) -> Option<Self> {
        let vec: Vec<_> = reference_name.split("_").collect();
        vec.last().map(|idx|idx.parse::<usize>().ok()).flatten().and_then(|idx|ReferenceMetaInfo::get_from_global(idx))
    }
    pub fn put_to_global(self) {
        let meta_infos = REFERENCE_META_INFO.get_or_init(|| {
            let  v = Vec::new();
            Arc::new(Mutex::new(v))
        });
    
        let mut meta_infos = meta_infos.lock().expect("can not get reference meta info lock");
        meta_infos.insert(self.idx, self);
    }
}

impl Default for ReferenceMetaInfo {
    fn default() -> Self {
        let idx = REFERENCE_META_INFO_G_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Self {
            interface: None,
            serialization: Serialization::Json,
            deserialization: Deserialization::Json,
            idx,
        }
    }
}

impl syn::parse::Parse for ReferenceMetaInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut interface = None;
        let mut serialization = Serialization::Json;
        let mut deserialization = Deserialization::Json;
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: syn::LitStr = input.parse()?;
            let value = value.value();
            match key.to_string().as_str() {
                "interface" => {
                    interface = Some(value);
                }
                "ser" => {
                    serialization = Serialization::from(value);
                }
                "deser" => {
                    deserialization = Deserialization::from(value);
                }
                _ => {
                    return Err(syn::Error::new(key.span(), "unknown attribute"));
                }
            }
            
            let _ = input.parse::<Token![,]>();
        }
       
        let mut meta_info = ReferenceMetaInfo::default();
        meta_info.interface = interface;
        meta_info.serialization = serialization;
        meta_info.deserialization = deserialization;
        
        
        Ok(meta_info)
    }
}
