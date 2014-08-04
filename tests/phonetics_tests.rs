extern crate natural;

use natural::phonetics::soundex;
use natural::phonetics::SoundexWord;
use std::collections::HashMap;

#[test]
fn test_soundex_methods() {
  assert!(soundex("rupert", "robert"));
  let s1 = SoundexWord::new("rupert");
  let s2 = SoundexWord::new("robert");
  assert!(s1.sounds_like(s2));
  assert!(s1.sounds_like_str("robert"));
}

#[test]
fn test_soundex_algorithm() {
  let mut cases = HashMap::new();
  cases.insert("rubin", "raven");
  cases.insert("rubin", "ripen");
  cases.insert("rubin", "riven");
  cases.insert("hello", "hallo");
  cases.insert("hello", "holey");
  cases.insert("hello", "hilly");
  cases.insert("hello", "hilly");
  cases.insert("BILL", "BAILEY");
  cases.insert("BILL", "BAILLIE");
  cases.insert("BILL", "BAILLY");
  cases.insert("BILL", "BAILY");
  cases.insert("BILL", "BALA");
  cases.insert("nguyen", "nixon");
  
  for (w1,w2) in cases.move_iter() {
    assert!(soundex(w1, w2));
  }
  
}