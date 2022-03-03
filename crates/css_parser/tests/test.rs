use css_parser::parse;
use insta::assert_debug_snapshot;


#[test]
fn test2() {
  let v = parse(".outer#bye {
      background: #00ccff;
      border-color: #666666;
      border-width: 2px;
      margin: 50px;
      padding: 50px;
    }"
    .to_string(),
  ); 
  assert_debug_snapshot!(v)
  // assert_eq!('.', m)
}
