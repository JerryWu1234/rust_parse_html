use css_parser;
use insta::assert_debug_snapshot;

#[test]
fn test1() {
  let mut v = css_parser::Parser {
    pos: 0,
    input: " .div {}".to_string(),
  };
  v.consume_whitespace();
  let m = v.consume_char();
  assert_eq!('.', m)
}

#[test]
fn test2() {
  let v = css_parser::Parser {
    pos: 0,
    input: ".outer {
      background: #00ccff;
      border-color: #666666;
      border-width: 2px;
      margin: 50px;
      padding: 50px;
    }"
    .to_string(),
  }
  .parse_rules();
  assert_debug_snapshot!(v)
  // assert_eq!('.', m)
}
