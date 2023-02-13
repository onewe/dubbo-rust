
use dubbo_extension::{RegistryExtensionWrapper, RegistryExtension};


#[repr(C)]
pub struct NacosRegistry {
    name: String
}


impl RegistryExtension for NacosRegistry {
    fn register(&self) {
       println!("{} registry invoke method", &self.name);
    }
}

#[no_mangle] 
pub unsafe extern "C" fn dubbo_extension() -> *mut RegistryExtensionWrapper {

    let nacos_registry = NacosRegistry {name: "nacos".to_string()};
    let nacos_registry = Box::new(nacos_registry);
    let extension = Box::new(RegistryExtensionWrapper::new(nacos_registry));
    let extension = Box::into_raw(extension);
    extension
}