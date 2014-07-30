extern crate natural;

use natural::tokenize::tokenize;

#[test]
fn test_tokenize() {
  assert_eq!(tokenize("hello, world!"), vec!["hello", "world"]);
  assert_eq!(tokenize("My dog has fleas."), vec!["My", "dog", "has", "fleas"]);
}