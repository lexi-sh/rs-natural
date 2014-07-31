extern crate natural;

use natural::ngram::get_ngram;
use natural::ngram::get_ngram_with_padding;

#[test]
fn test_ngram() {
  assert_eq!(get_ngram("hello my darling", 2), vec![vec!["hello", "my"], vec!["my", "darling"]]);
}

#[test]
fn test_ngram_with_pad() {
  assert_eq!(get_ngram_with_padding("my fleas", 2, "boo"), vec![
    vec!["boo", "my"], vec!["my", "fleas"], vec!["fleas", "boo"]]);
}