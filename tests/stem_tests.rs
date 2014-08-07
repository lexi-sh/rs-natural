extern crate natural;

use natural::stem::get;
use std::collections::HashMap;

#[test]
fn test_stem() {
  let mut cases = HashMap::new();
  cases.insert("a", "a");
  cases.insert("aaron", "aaron");
  cases.insert("abaissiez", "abaissiez");
  cases.insert("abandon", "abandon");
  cases.insert("abandoned", "abandon");
  cases.insert("abase", "abas");
  cases.insert("abash", "abash");
  
  for (input,output) in cases.move_iter() {
    let answer = get(input);
    assert_eq!(answer.as_slice(), output);
  }
}