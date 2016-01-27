extern crate stem;

use tokenize::tokenize;
use stem::get;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct NaiveBayesClassifier {
  documents: HashMap<String, HashMap<String, usize>>,
  total_document_count: usize,
}

impl NaiveBayesClassifier {
  pub fn new() -> NaiveBayesClassifier {
    NaiveBayesClassifier {
      documents: HashMap::new(),
      total_document_count: 0,
    }
  }
  
  pub fn train(&mut self, text: String, classification: String) {
    let classification_map = self.documents.entry(classification)
                                           .or_insert_with(|| HashMap::new());
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    for stemmed_word in stemmed_and_tokenized.into_iter() {
      let stemmed_word_entry = classification_map.entry(stemmed_word).or_insert(1);
      *stemmed_word_entry += 1;
    }
    self.total_document_count += 1;
  }
  
  pub fn guess(&self, text: &str) -> String {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    let mut label_probabilities = HashMap::new();
    for (k, v) in self.documents.iter() {
      //Get the probability that the passed-in text is each class
      let mut probability = 0.0f32;
      for stemmed_word in &stemmed_and_tokenized {
        if v.contains_key(stemmed_word) {
          probability += (1.0 / v.len() as f32).ln();
        }
      }
      if probability.abs() < 0.0001 {
        label_probabilities.insert(k, 0.0);
      } else {
        label_probabilities.insert(k, (v.len() as f32 * probability.abs() / self.total_document_count as f32));
      }
    }
    
    let mut answer_label = String::new();
    let mut answer_probability = 0.0;
    for (k, v) in label_probabilities.into_iter() {
      if answer_probability <= v {
        answer_label = k.clone();
        answer_probability = v;
      }
    }
    answer_label
  }
}

fn get_tokenized_and_stemmed(text: &str) -> Vec<String> {
  let tokenized_text = tokenize(text);
  tokenized_text.iter()
                .map(|idx| stem::get(tokenized_text[idx]).unwrap())
                .collect()
}
