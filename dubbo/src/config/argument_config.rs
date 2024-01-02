
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArgumentConfig {
    index: i32,
    r#type: String,
}

impl ArgumentConfig {
    pub fn new(index: i32, r#type: String) -> Self {
        Self {
            index,
            r#type,
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn index_mut(&mut self) -> &mut i32 {
        &mut self.index
    }

    pub fn r#type_mut(&mut self) -> &mut str {
        &mut self.r#type
    }
}