#[derive(Debug)]
pub struct Stylesheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
  pub selectors: Vec<Selector>,
  pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
  Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
  pub tag_name: Option<String>,
  pub id: Option<String>,
  pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
  pub name: String,
  pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
  Px,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}
pub fn parse(source: String) -> Stylesheet {
  let mut parser = Parser {
    pos: 0,
    input: source,
  };
  Stylesheet {
    rules: parser.parse_rules(),
  }
}
pub struct Parser {
  pub pos: usize,
  pub input: String,
}

impl Selector {
  pub fn speificity(&self) -> (usize, usize, usize) {
    let Selector::Simple(valeu) = self;
    let a = valeu.id.iter().count();
    let v = valeu.class.len();
    let c = valeu.tag_name.iter().count();
    (a, v, c)
  }
}

impl Parser {
  pub fn parse_rules(&mut self) -> Vec<Rule> {
    let mut sheet: Vec<Rule> = vec![];
    loop {
      self.consume_whitespace();
      if self.eof() {
        break;
      }
      sheet.push(self.parse_rule())
    }
    sheet
  }
  pub fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: self.parse_selectors(),
      declarations: self.parse_declarations(),
    }
  }

  pub fn parse_declarations(&mut self) -> Vec<Declaration> {
    assert_eq!(self.consume_char(), '{');
    let mut declarations: Vec<Declaration> = vec![];
    loop {
      self.consume_whitespace();
      if self.next_char() == '}' {
        self.consume_char();
        break;
      }
      declarations.push(self.parse_declaration());
    }
    declarations
  }

  pub fn parse_declaration(&mut self) -> Declaration {
    let name = self.iteration(getValue);
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ':');
    self.consume_whitespace();
    let value = self.parse_value();
    self.consume_whitespace();
    assert_eq!(self.consume_char(), ';');
    Declaration {
      name: name,
      value: value,
    }
  }

  pub fn parse_value(&mut self) -> Value {
    match self.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => Value::Keyword(self.iteration(getValue)),
    }
  }

  pub fn parse_color(&mut self) -> Value {
    assert_eq!(self.consume_char(), '#');
    Value::ColorValue(Color {
      r: self.parse_hex_pair(),
      g: self.parse_hex_pair(),
      b: self.parse_hex_pair(),
      a: 255,
    })
  }

  pub fn parse_hex_pair(&mut self) -> u8 {
    let s = &self.input[self.pos..self.pos + 2];
    self.pos += 2;
    u8::from_str_radix(s, 16).unwrap()
  }

  pub fn parse_length(&mut self) -> Value {
    let value = self.iteration(|c| match c {
      '0'..='9' => true,
      _ => false,
    });

    let unit = self.iteration(getValue).to_ascii_lowercase();

    Value::Length(
      value.parse().unwrap(),
      match &*unit {
        "px" => Unit::Px,
        _ => panic!("unrecognized unit"),
      },
    )
  }
  pub fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = vec![];
    loop {
      selectors.push(Selector::Simple(self.parse_simple_selector()));
      self.consume_whitespace();
      match self.next_char() {
        ',' => {
          self.consume_char();
          self.consume_whitespace();
        }
        '{' => break,
        c => panic!("Unexpected character {} in selector list", c),
      }
    }
    selectors.sort_by(|a, b| b.speificity().cmp(&a.speificity()));
    selectors
  }

  pub fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {
      tag_name: None,
      id: None,
      class: vec![],
    };
    while !self.eof() {
      match self.next_char() {
        '#' => {
          self.consume_char();
          selector.id = Some(self.iteration(getValue))
        }
        '.' => {
          self.consume_char();
          selector.class.push(self.iteration(getValue))
        }
        '*' => {
          self.consume_char();
        }
        c if getValue(c) => selector.tag_name = Some(self.iteration(getValue)),
        _ => break,
      }
    }
    selector
  }

  pub fn next_char(&mut self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  pub fn consume_whitespace(&mut self) {
    self.iteration(char::is_whitespace);
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
    println!("current_char:{}", char);
    char
  }
  pub fn eof(&mut self) -> bool {
    self.pos >= self.input.len()
  }
}

fn getValue(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}
