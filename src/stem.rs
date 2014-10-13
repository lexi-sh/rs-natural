extern crate stem;

pub fn stem<T:Str>(word: T) -> String {
  stem::get(word.as_slice()).unwrap()
}