#[cfg(test)]
mod tests {
    use dubbo_extension::{RegistryExtensionWrapper, RegistryExtension};
    use libloading::*;

    #[test]
    fn load_lib(){
        let reg = unsafe {
            let lib = Library::new("./lib/libnacos_extension.dylib").unwrap();
            let func: libloading::Symbol<unsafe extern fn() -> *mut RegistryExtensionWrapper> = lib.get(b"dubbo_extension").unwrap();
            let reg = func();
            let reg = Box::from_raw(reg);
             reg
        };
        
        reg.register();
    }
}