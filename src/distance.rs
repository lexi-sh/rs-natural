use std::cmp;

fn max_length(str1: &str, str2: &str) -> usize {
    if str1.len() > str2.len() {
      str1.len()
    } else {
      str2.len()
    }
}

fn get_common_chars(str1: &str, str2: &str, ordering: bool) -> Vec<char> {
    let mut common_chars = Vec::new();

    if str1.len() == 0 || str2.len() == 0 {
        return common_chars;
    }

    let max_length = max_length(str1, str2);
    let match_buffer = ( max_length / 2 ) - 1;
    let mut f = str1;
    let mut s = str2;

    if ordering {
        f = str2;
        s = str1;
    }

    for (i, c) in f.chars().enumerate() {
        let s_index: usize = if i < match_buffer {
            0
        } else {
            i - match_buffer
        };
      
        let e_index: usize = if s.len() <= (i + match_buffer) { 
            s.len() 
        } else { 
            i + match_buffer + 1
        };
          
        let word_slice = &s[s_index..e_index];
        if word_slice.contains(c) {
            common_chars.push(c);
        }
    }

    common_chars
}
  
pub fn jaro_winkler_distance(str1: &str, str2: &str) -> f32 {
    let matches1 = get_common_chars(str1, str2, false);
    let matches2 = get_common_chars(str1, str2, true);

    if matches1.is_empty() || matches2.is_empty()
      || matches1.len() != matches2.len()
    {
      return 0.0;
    }
    
    let mut t = 0.0;
    for (a, b) in matches1.iter().zip(matches2.iter()) {
      if a != b {
        t += 0.5;
      }
    }
    
    let f1 = matches1.len() as f32 / str1.len() as f32;
    let f2 = matches2.len() as f32 / str2.len() as f32;
    let f3 = (matches1.len() as f32 - t) / matches1.len() as f32;
    (f1 + f2 + f3) / 3.0
}

/* pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    // TODO: unimplemented
}*/
