extern crate stem;

pub fn stem<'a>(word: &'a str) -> String {
  stem::get(word).unwrap()
}