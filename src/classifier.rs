use std::collections::HashMap;
use tokenize::tokenize;
use std::borrow::Cow;
use rust_stemmers::{Algorithm, Stemmer};

#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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
  
  // Add counts of terms in some text to a classification
  pub fn train(&mut self, text: &str, classification: &str) {
    let classification_map = self.documents.entry(classification.to_string())
                                           .or_default();
    get_tokenized_and_stemmed(text).into_iter()
        .for_each(|token| {
            classification_map.entry(token.to_string()).and_modify(|e| *e += 1).or_insert(1);
        });
    self.total_document_count += 1;
  }
  
  // Get a guess of input text based on existing unigram counts
  pub fn guess(&self, text: &str) -> String {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);

    self.documents.iter()
        .map(|(class, word_counts)| {
            let probability: f64 = stemmed_and_tokenized.iter()
                .filter(|token| word_counts.contains_key(&token.to_string()))
                .map(|_| {
                    (1.0 / word_counts.len() as f64).ln()
                }).sum();

            let prob_abs = probability.abs();
            let normalized_prob = if prob_abs < 0.0001 {
                0.0
            } else {
                word_counts.len() as f64 * prob_abs / self.total_document_count as f64
            };
            
            (class, normalized_prob)
        }).max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).expect("failed to ").0.clone()
  }
}

fn get_tokenized_and_stemmed<'a>(text: &'a str) -> Vec<Cow<'a, str>> {
  let en_stemmer = Stemmer::create(Algorithm::English);
  tokenize(text).into_iter()
                .map(|text| en_stemmer.stem(text))
                .collect()
}
