use std::collections::HashMap;

#[derive(Default, Debug)]
pub(crate) struct ApplicationConfig {
    name: String,
    parameters: HashMap<String, String>,
}

impl ApplicationConfig {
    pub(crate) fn new(name: String, parameters: HashMap<String, String>) -> Self {
        Self {
            name,
            parameters,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub(crate) fn set_parameters(&mut self, parameters: HashMap<String, String>) {
        self.parameters = parameters;
    }

    pub(crate) fn add_parameter(&mut self, key: String, value: String) {
        self.parameters.insert(key, value);
    }
}