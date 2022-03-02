use html_parser::dom;
use html_parser::{Parser, parse};
use insta::assert_debug_snapshot;

#[test]
fn it_works() {
    let m = dom::create_text("example".to_string());
    assert_debug_snapshot!(m);
}

#[test]
fn it_parser() {
    let c = Parser {
        pos: 1,
        input: "<div></div>".to_string(),
    }
    .next_char();
    assert_eq!('d', c)
}

#[test]
fn testiterator() {
    let mut c = Parser {
        pos: 0,
        input: "   <div></div>".to_string(),
    };

    c.iteration(|c| match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' => false,
        _ => true,
    });
    let v = c.consume_char();
    assert_eq!('d', v)
}

#[test]
fn test_attribute() {
    let mut c = parse("<html>
        <head>
          <title>Test</title>
        </head>
        <div class=\"outer\">
          <p class=\"inner\">
            Hello, <span id=\"name\">world!</span>
          </p>
          <p class=\"inner\" id=\"bye\">
            Goodbye!
          </p>
        </div>
      </html>".to_string());
    assert_debug_snapshot!(c);
}
