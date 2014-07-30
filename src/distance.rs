struct CalcObjects<'a> {
  first: &'a str,
  second: &'a str,
}

impl<'a> CalcObjects<'a> {
  fn max_length(&self) -> uint {
    return if self.first.len() > self.second.len() {
      self.first.len()
    }
    else {
      self.second.len()
    };
  }
}

struct JaroWinkler<'a> {
  co: CalcObjects<'a>,
  matches1: Vec<char>,
  matches2: Vec<char>
}

impl<'b> JaroWinkler<'b> {
  fn new<'b> (this_obj: CalcObjects<'b>) -> JaroWinkler<'b> {
    let mut jw = JaroWinkler {
      co: this_obj,
      matches1: Vec::new(),
      matches2: Vec::new()
    };
    jw.matches1 = jw.get_common_chars(false);
    jw.matches2 = jw.get_common_chars(true);
    jw
  }
  
  fn get_common_chars(&self, ordering: bool) -> Vec<char> {
    let mut common_chars = Vec::new();
    let max_length = self.co.max_length();
    let match_buffer = ( max_length / 2 ) - 1;
    let mut f = self.co.first;
    let mut s = self.co.second;
    if ordering {
      f = self.co.second;
      s = self.co.first;
    }
    
    if f.len() == 0 || s.len() == 0 {
      return common_chars;
    }
    
    for (i,c) in f.chars().enumerate() {
      let s_index: uint = if i < match_buffer {
        0
      }
      else {
        i - match_buffer
      };
      let e_index: uint = if s.len() <= i + match_buffer { 
        s.len() 
      }
      else { 
        i + match_buffer + 1
      };
      let word_slice = s.slice(s_index, e_index);
      if word_slice.contains_char(c) {
        common_chars.push(c);
      }
    }
    common_chars
  }
  
  fn calculate(&self) -> f32 {
    if self.matches1.len() == 0 || self.matches2.len() == 0 {
      return 0.0;
    }
    if self.matches1.len() != self.matches2.len() {
      return 0.0;
    }
    
    let mut t = 0.0;
    let n = self.matches1.len();
    for i in range(0u,n) {
      if self.matches2[i] != self.matches1[i] {
        t += 0.5;
      }
    }
    let f1: f32 = self.matches1.len() as f32 / self.co.first.len() as f32;
    let f2: f32 = self.matches2.len() as f32 / self.co.second.len() as f32;
    let f3: f32 = (self.matches1.len() as f32 - t) / self.matches1.len() as f32;
    ( ( f1 + f2 + f3 ) / 3.0 )
  }
}

pub fn jaro_winkler_distance<'sl>(str1: &'sl str, str2: &'sl str) -> f32 {
  let jw = JaroWinkler::new(CalcObjects{ first: str1, second: str2 });
  jw.calculate()
}

pub fn levenshtein_distance<'sl>(str1: &'sl str, str2: &'sl str) -> uint {
  str1.lev_distance(str2)
}