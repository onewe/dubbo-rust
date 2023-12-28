
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ArgumentConfig {
    index: i32,
    r#type: String,
}

impl ArgumentConfig {
    pub(crate) fn new(index: i32, r#type: String) -> Self {
        Self {
            index,
            r#type,
        }
    }

    pub(crate) fn index(&self) -> i32 {
        self.index
    }

    pub(crate) fn r#type(&self) -> &str {
        &self.r#type
    }

    pub(crate) fn index_mut(&mut self) -> &mut i32 {
        &mut self.index
    }

    pub(crate) fn r#type_mut(&mut self) -> &mut str {
        &mut self.r#type
    }
}