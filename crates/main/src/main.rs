use std::{fs::File, io::Read};

// use css_parser;
use html_parser;
// use style;
fn main() {
  let html = read_sourcr("example/test.html".to_string());
  let css = read_sourcr("example/test.css".to_string());
  let sourcecss = css_parser::parse(css);
  let sourcehtml = html_parser::parse(html);
  let styletree = style::style_tree(&sourcehtml, &sourcecss);
  println!("》》》》》》》{:#?}",styletree)

  
}

fn read_sourcr(file_name: String) -> String {
  let mut str = String::new();
  File::open(file_name)
    .unwrap()
    .read_to_string(&mut str)
    .unwrap();
  str
}
