use graph_items::edge::Edge;
use graph_items::node::Node;
use std::collections::HashMap;

pub mod graph_items;

pub fn convert_attrs(attrs: &[(&str, &str)]) -> HashMap<String, String> {
    attrs
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { ..Self::default() }
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes.extend(nodes.iter().cloned());
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges.extend(edges.iter().cloned());
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs.extend(convert_attrs(attrs));
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|x| x.name == name)
    }
}
