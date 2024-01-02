use super::argument_config::ArgumentConfig;

#[derive(Default, Debug)]
pub struct MethodConfig {
    
    name: String,
    
    arguments: Vec<ArgumentConfig>,
}

impl MethodConfig {

    pub fn new(name: String, arguments: Vec<ArgumentConfig>) -> Self {
        Self {
            name,
            arguments,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn arguments(&self) -> &Vec<ArgumentConfig> {
        &self.arguments
    }

    pub fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub fn arguments_mut(&mut self) -> &mut Vec<ArgumentConfig> {
        &mut self.arguments
    }

    pub fn add_argument(&mut self, argument: ArgumentConfig) {
        self.arguments.push(argument);
    }
}