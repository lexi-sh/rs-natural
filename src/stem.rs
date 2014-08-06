extern crate stem;

pub fn get<'a>(word: &'a str) -> String {
  stem::get(word).unwrap()
}