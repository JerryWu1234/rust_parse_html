pub mod dom;
// use std::{collections::HashMap, hash::Hash};

use std::{collections::HashMap, hash::Hash};

pub use dom::*;

#[derive(Debug)]
pub struct Parser {
  pub pos: usize,
  pub input: String,
}

impl Parser {
  pub fn parse_nodes(&mut self) -> Vec<Node> {
    let mut node: Vec<Node> = vec![];
    loop {
      self.consume_whitespace();
      println!(">>{:?}", self.input[self.pos..].to_string());
      if self.eof() || self.input[self.pos..].starts_with("</") {
        break;
      }
      node.push(self.parse_node())
    }
    node
  }

  pub fn parse_node(&mut self) -> Node {
    match self.next_char() {
      '<' => self.cusume_element(),
      _ => self.cusume_text(),
    }
  }

  pub fn cusume_text(&mut self) -> Node {
    dom::create_text(self.iteration(|c| c != '>'))
  }

  pub fn cusume_element(&mut self) -> Node {
    assert_eq!('<', self.consume_char());
    let tag_name = self.tag_name();
    self.consume_whitespace();
    let attributes = self.consume_attributes();
    let children = self.parse_nodes();
    dom::create_element(tag_name, attributes, children)
  }

  pub fn consume_attributes(&mut self) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    loop {
      self.consume_whitespace();
      if self.consume_char() == '>' {
        break;
      }
      let (key, value) = self.get_attributes();
      attrs.insert(key, value);
    }
    attrs
  }

  pub fn get_attributes(&mut self) -> (String, String) {
    let key = self.iteration(getStrings);
    assert_eq!('=', self.consume_char());
    let open_quote = self.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');
    let value = self.iteration(|c| c != open_quote);
    assert_eq!('"', self.consume_char());

    (key, value)
  }

  pub fn tag_name(&mut self) -> String {
    self.iteration(getStrings)
  }

  pub fn consume_whitespace(&mut self) {
    self.iteration(char::is_whitespace);
  }

  pub fn next_char(&mut self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  pub fn iteration<F>(&mut self, callback: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut value = String::from("");
    while !self.eof() && callback(self.next_char()) {
      value.push(self.consume_char())
    }
    value
  }

  pub fn consume_char(&mut self) -> char {
    let (index, char) = self.input[self.pos..].char_indices().next().unwrap();
    self.pos += index + 1;
    println!("{},>>>打印当前循环的char", char);
    char
  }

  pub fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }
}

fn getStrings(v: char) -> bool {
  match v {
    'a'..='z' | 'A'..='Z' | '0'..='9' => true,
    _ => false,
  }
}
