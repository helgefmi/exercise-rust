use crate::graph::convert_attrs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Edge {
    left: String,
    right: String,
    attrs: HashMap<String, String>
}

impl Edge {
    pub fn new(left: &str, right: &str) -> Edge {
        Edge {
            left: left.to_string(),
            right: right.to_string(),
            ..Self::default()
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend(convert_attrs(attrs));
        self
    }
}
