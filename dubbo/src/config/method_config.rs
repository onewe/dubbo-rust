use super::argument_config::ArgumentConfig;

#[derive(Default, Debug)]
pub(crate) struct MethodConfig {
    
    name: String,
    
    arguments: Vec<ArgumentConfig>,
}

impl MethodConfig {

    pub(crate) fn new(name: String, arguments: Vec<ArgumentConfig>) -> Self {
        Self {
            name,
            arguments,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn arguments(&self) -> &Vec<ArgumentConfig> {
        &self.arguments
    }

    pub(crate) fn name_mut(&mut self) -> &mut str {
        &mut self.name
    }

    pub(crate) fn arguments_mut(&mut self) -> &mut Vec<ArgumentConfig> {
        &mut self.arguments
    }

    pub(crate) fn add_argument(&mut self, argument: ArgumentConfig) {
        self.arguments.push(argument);
    }
}