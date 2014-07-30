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
    
    }
    
    //Fill the rest of the ngram
    for i in range(0, count) {
      let sl = Vec::from_slice(tokenized_sequence.slice(i,i+self.n));
      ngram_result.push(sl);
    }
    
    //right-padding
    if self.pad != "" {
    
    }
    ngram_result
  }
}

pub fn get_ngram<'a>(this_text: &'a str, this_n: uint) -> Vec<Vec<&'a str>> {
  let ng = NGram { text: this_text, n: this_n, pad: "" };
  ng.calculate()
}

pub fn get_ngram_with_padding<'a>(this_text: &'a str, this_n: uint, this_padding: &'a str)  {
  
}