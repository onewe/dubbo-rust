
#[derive(Clone, Default)]
pub struct InvokerDirectoryConfig {
    r_type: String,
}

impl InvokerDirectoryConfig {

    pub fn invoker_directory_type(&self) -> &str {
        &self.r_type
    }

    pub fn invoker_directory_type_mut(&mut self) -> &mut String {
        &mut self.r_type
    }
}