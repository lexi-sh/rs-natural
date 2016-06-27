use tokenize::tokenize;

struct NGram<'a> {
  text: &'a str,
  n: usize,
  pad: &'a str
}

impl<'a> NGram<'a> {
  fn calculate(&self) -> Vec<Vec<&'a str>> {
    let mut tokenized_sequence = tokenize(self.text);
    tokenized_sequence.shrink_to_fit();
    
    let count = tokenized_sequence.len() - self.n + 1;
    
    let mut ngram_result = Vec::new();
    
    //left-padding
    if !self.pad.is_empty() {
      for i in 1..self.n {
        let num_blanks = self.n - i;
        let mut this_sequence = Vec::new();
        for _ in 0..num_blanks {
          this_sequence.push(self.pad);
        }
        let sl = &tokenized_sequence[0 .. (self.n - num_blanks)];
        this_sequence.extend_from_slice(sl);
        ngram_result.push(this_sequence);
      }
    }
    
    //Fill the rest of the ngram
    for i in 0..count {
      let a = &tokenized_sequence[i..i + self.n];
      let sl = a.to_vec();
      ngram_result.push(sl);
    }
    
    //right-padding
    if !self.pad.is_empty() {
      for i in 1..self.n {
        let num_blanks = i;
        let last_entry = tokenized_sequence.len();
        let mut tc = Vec::new();
        tc.extend_from_slice(&tokenized_sequence[(last_entry - num_blanks) .. last_entry]);
        for _ in 0..num_blanks {
          tc.push(self.pad);
        }
        ngram_result.push(tc);
      }
    }
    ngram_result
  }
}

pub fn get_ngram<'a>(this_text: &'a str, this_n: usize) -> Vec<Vec<&'a str>> {
  let ng = NGram { text: this_text, n: this_n, pad: "" };
  ng.calculate()
}

pub fn get_ngram_with_padding<'a>(this_text: &'a str, this_n: usize, this_padding: &'a str) -> Vec<Vec<&'a str>> {
  let ng = NGram { text: this_text, n: this_n, pad: this_padding };
  ng.calculate()
}
