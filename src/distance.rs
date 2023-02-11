use std::cmp;
extern crate unicode_segmentation;
use self::unicode_segmentation::UnicodeSegmentation;

fn max_length(str1: &str, str2: &str) -> usize {
    if str1.len() > str2.len() {
      str1.len()
    } else {
      str2.len()
    }
}

fn get_common_chars(str1: &str, str2: &str, ordering: bool) -> Vec<char> {
    let mut common_chars = Vec::new();

    if str1.is_empty() || str2.is_empty() {
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

// ported from http://hetland.org/coding/python/levenshtein.py
pub fn levenshtein_distance(str1: &str, str2: &str) -> usize {
    let a_vec:Vec<&str> = UnicodeSegmentation::graphemes(str1, true);
    let b_vec:Vec<&str> = UnicodeSegmentation::graphemes(str2, true);
    let n = a_vec.len();
    let m = b_vec.len();

    let mut column: Vec<usize> = (0..=n).collect();    
    for i in 1..=m {
        let previous = column;
        column = vec![0; n + 1];
        column[0] = i;
        for j in 1..=n {
            let add = previous[j] + 1;
            let delete = column[j - 1] + 1;
            let mut change = previous[j - 1];
            if a_vec[j - 1] != b_vec[i - 1] {
                change += 1
            }
            column[j] = min3(add, delete, change);
        }
    }
    column[n]
}

fn min3<T: Ord>(a: T, b: T, c: T) -> T{
    cmp::min(a, cmp::min(b, c))
}
