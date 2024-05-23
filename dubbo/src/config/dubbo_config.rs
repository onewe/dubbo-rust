

#[derive(Clone)]
pub struct DubboConfig;


impl DubboConfig {

    pub fn cluster(&self) -> String {
        "test".to_string()
    }
}