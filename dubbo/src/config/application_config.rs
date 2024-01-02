use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ApplicationConfig {
    name: String,
    parameters: HashMap<String, String>,
}

impl ApplicationConfig {
    pub fn new(name: String, parameters: HashMap<String, String>) -> Self {
        Self {
            name,
            parameters,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_parameters(&mut self, parameters: HashMap<String, String>) {
        self.parameters = parameters;
    }

    pub fn add_parameter(&mut self, key: String, value: String) {
        self.parameters.insert(key, value);
    }
}