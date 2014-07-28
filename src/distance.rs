struct CalcObjects<'a> {
  first: &'a str,
  second: &'a str
}
pub fn jaro_winkler_distance<'sl>(str1: &'sl str, str2: &'sl str) -> f32 {
  let co = CalcObjects{ first: str1, second: str2 };
  let m = get_num_matching_chars(&co);
  let t = get_num_transpositions(&co) / 2.0;
  if m < 1.0 {
    0.0
  }
  else {
    let f1: f32 = m / str1.len() as f32;
    let f2: f32 = m / str2.len() as f32;
    let f3: f32 = (m - t) / m;
    ( ( f1 + f2 + f3 ) / 3.0 )
  }
}

fn get_num_matching_chars(obj: &CalcObjects) -> f32 {
  let mut m = 0.0;
  
  for (i,c) in obj.first.chars().enumerate() {
    if i >= obj.second.len() {
      break;
    }
    if c == obj.second.char_at(i) {
      m += 1.0;
    }
  }
  m
}

fn get_num_transpositions(obj: &CalcObjects) -> f32 {
  let mut t = 0.0;
  let max_length =
    if obj.first.len() > obj.second.len() {
      obj.first.len()
    }
    else {
      obj.second.len()
    };
  let match_buffer = ( max_length / 2 ) - 1;
  
  for (i,c) in obj.first.chars().enumerate() {
    if i >= obj.second.len() {
      break;
    }
    if c != obj.second.char_at(i) {
      if is_transposed(obj.second, i, match_buffer, c) {
        t += 1.0;
      }
    }
  }
  t
}

fn is_transposed<'a>(word: &'a str, index: uint, buffer: uint, matching_char: char) -> bool {
  let start_index: uint = if index < buffer { 
      0 
    } 
    else { 
      index - buffer 
    };
    
  let end_index: uint = if word.len() >= index + buffer { 
      word.len() - 1 
    }
    else { 
      index + buffer 
    };

  for (i,c) in word.slice(start_index,end_index).chars().enumerate() {
    if c == matching_char {
      return true;
    }
  }
  return false;
}
