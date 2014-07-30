pub fn tokenize<'a>(text: &'a str) -> Vec<&str> {
  let vec_with_empty: Vec<&str> = text.split(|c: char| char_is_token(c)).collect();
  let mut ret_vec = Vec::new();
  for s in vec_with_empty.move_iter() {
    if s.len() > 0 {
      ret_vec.push(s);
    }
  }
  ret_vec
}

fn char_is_token(a: char) -> bool {
  let tokens = vec![' ', ',', '.', '!', '?', ';', '\'', '"', ':', '\t', '\n', '(', ')', '-'];
  for x in tokens.iter() {
    if a == *x {
      return true;
    }
  }
  return false;
}