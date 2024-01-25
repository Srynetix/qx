use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Context(pub HashMap<String, String>);

impl Context {
    pub fn new(values: HashMap<String, String>) -> Self {
        Self(values)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}
