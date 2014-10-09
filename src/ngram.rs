use tokenize::tokenize;

struct NGram<'a> {
  text: &'a str,
  n: uint,
  pad: &'a str
}

impl<'a> NGram<'a> {
  fn calculate(&self) -> Vec<Vec<&'a str>> {
    
    let mut tokenized_sequence = tokenize(self.text);
    tokenized_sequence.shrink_to_fit();
    
    let count = tokenized_sequence.len() - self.n + 1;
    
    let mut ngram_result = Vec::new();
    
    //left-padding
    if self.pad != "" {
      for i in range(1, self.n) {
        let num_blanks = self.n - i;
        let mut this_sequence = Vec::new();
        for _j in range(0, num_blanks) {
          this_sequence.push(self.pad);
        }
        let sl = tokenized_sequence.slice(0, self.n-num_blanks);
        this_sequence.push_all(sl);
        ngram_result.push(this_sequence);
      }
    }
    
    //Fill the rest of the ngram
    for i in range(0, count) {
      let a = tokenized_sequence.slice(i,i+self.n);
      let sl = a.to_vec();
      ngram_result.push(sl);
    }
    
    //right-padding
    if self.pad != "" {
      for i in range(1, self.n) {
        let num_blanks = i;
        let last_entry = tokenized_sequence.len();
        let mut tc = Vec::new();
        tc.push_all(tokenized_sequence.slice(last_entry - num_blanks, last_entry));
        for _j in range(0, num_blanks) {
          tc.push(self.pad);
        }
        ngram_result.push(tc);
      }
    }
    ngram_result
  }
}

pub fn get_ngram<'a>(this_text: &'a str, this_n: uint) -> Vec<Vec<&'a str>> {
  let ng = NGram { text: this_text, n: this_n, pad: "" };
  ng.calculate()
}

pub fn get_ngram_with_padding<'a>(this_text: &'a str, this_n: uint, this_padding: &'a str) -> Vec<Vec<&'a str>> {
  let ng = NGram { text: this_text, n: this_n, pad: this_padding };
  ng.calculate()
}