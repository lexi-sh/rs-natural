//Input: two words
//Output: True if the two words song alike
pub fn soundex(word1: &str, word2: &str) -> bool {
  let sword1 = SoundexWord::new(word1);
  let sword2 = SoundexWord::new(word2);
  sword1.word == sword2.word
}



pub struct SoundexWord {
  word: Vec<char>
}

impl SoundexWord {

  pub fn new(word: &str) -> SoundexWord {
    let mut chars: Vec<char> = Vec::new();
    
    for c in word.chars() {
      chars.push(c);
    }
    chars = soundex_encoding(chars);
    SoundexWord { word: chars }
    
  }
  
  pub fn sounds_like(&self, second_word: SoundexWord) -> bool {
    self.word == second_word.word
  }
  
  pub fn sounds_like_str(&self, second_word: &str) -> bool {
    let sec_word = SoundexWord::new(second_word);
    self.word == sec_word.word
  }
}

fn soundex_encoding(chars: Vec<char>) -> Vec<char> {
  fix_length(strip_similar_chars(chars))
}

fn strip_similar_chars(chars: Vec<char>) -> Vec<char> {
  let mut enc_chars = Vec::new();
  enc_chars.push(chars[0]);
  for i in 1..chars.len() {
    enc_chars.push(get_char_digit(chars[i]));
  }
  let mut chars_no_hw = Vec::new();
  let mut chars_no_vowels = Vec::new();
  for c in enc_chars.into_iter() {
    if c != '9' {
      chars_no_hw.push(c);
    }
  }
  chars_no_hw.dedup();
  for c in chars_no_hw.into_iter() {
    if c != '0' {
      chars_no_vowels.push(c);
    }
  }
  chars_no_vowels
}

fn fix_length(mut chars: Vec<char>) -> Vec<char> {
  match chars.len() {
    4 => chars,
    0...3 => add_more_zeros(chars),
    _ => { chars.truncate(4); chars} //truncate doesn't return self?
  }
}

fn add_more_zeros(chars: Vec<char>) -> Vec<char> {
  (0..4).map( |idx| { 
    if idx < chars.len() {
      chars[idx]
    }
    else {
      '0'
    }
  }).collect()
}


fn get_char_digit(c: char) -> char {
  match c {
    'b' | 'f' | 'p' | 'v' => '1',
    'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => '2',
    'd' | 't' => '3',
    'l' => '4',
    'm' | 'n' => '5',
    'r' => '6',
    'h' | 'w' => '9', //0 and 9 are removed later, this is just to separate vowels from h and w
    _ => '0' //Vowels
  }
}
