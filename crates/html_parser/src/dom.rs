use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
  pub node_type: NodeType,
}

#[derive(Debug)]
pub struct ElementPreproty {
  pub tag_name: String,
  pub attributes: HashMap<String, String>,
  pub children: Vec<Node>,
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
    node_type: (NodeType::Element(ElementPreproty {
      tag_name,
      attributes,
      children,
    })),
  }
}

pub fn create_text(text: String) -> Node {
  Node {
    node_type: (NodeType::Text(text)),
  }
}
