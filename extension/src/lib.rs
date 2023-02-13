pub trait RegistryExtension {

    fn register(&self);
    
}

#[repr(C)]
pub struct RegistryExtensionWrapper {
    inner: Box<dyn RegistryExtension>
}

impl RegistryExtensionWrapper {

    pub fn new(inner: Box<dyn RegistryExtension>) -> Self 
    {
        Self { inner }
    }
}

impl RegistryExtension for RegistryExtensionWrapper {

    fn register(&self) {
        self.inner.register();
    }
}
