use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Node {
  pub children: Vec<Node>,
  pub node_type: NodeType,
}

#[derive(Debug)]
pub struct ElementPreproty {
  pub tag_name: String,
  pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum NodeType {
  Text(String),
  Element(ElementPreproty),
}

pub fn create_element(
  tag_name: String,
  attributes: HashMap<String, String>,
  children: Vec<Node>,
) -> Node {
  Node {
    children,
    node_type: (NodeType::Element(ElementPreproty {
      tag_name,
      attributes,
    })),
  }
}

pub fn create_text(text: String) -> Node {
  Node {
    children: vec![],
    node_type: (NodeType::Text(text)),
  }
}

impl ElementPreproty {
  pub fn id(&self) -> Option<&String> {
    self.attributes.get("id")
  }

  pub fn classes(&self) -> HashSet<&str> {
    match self.attributes.get("class") {
      Some(classlist) => classlist.split(' ').collect(),
      None => HashSet::new(),
    }
  }
}
