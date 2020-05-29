use crate::graph::convert_attrs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Node {
    pub name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            ..Self::default()
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend(convert_attrs(attrs));
        self
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(|s| s.as_str())
    }
}
