use std::str::CharEq;

pub fn tokenize(text: &str) -> Vec<&str> {
  text.split(Splitter).filter(|s| s.len() > 0).collect()
}

struct Splitter;

impl CharEq for Splitter {
    fn matches(&mut self, c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
            | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
            _ => false
        }
    }

    // We're only matching ASCII chars, so we can use the faster impl.
    fn only_ascii(&self) -> bool { true }
}
