extern crate natural;

use natural::ngram::get_ngram;

#[test]
fn test_ngram() {
  println!("{}", get_ngram("hello my darling", 2));
  assert_eq!(get_ngram("hello my darling", 2), vec![vec!["hello", "my"], vec!["my", "darling"]]);
}