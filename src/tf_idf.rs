use std::collections::{HashMap, HashSet};
use tokenize::tokenize;
use std::borrow::Cow;
use rust_stemmers::{Algorithm, Stemmer};

#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct TfIdf {
	//token counts for inverse document frequency
	doc_freqs: HashMap<String, usize>, 

	//documents joined into a single HashMap
	pub term_freqs: HashMap<String, usize>,

	//total count of documents inserted
	doc_count: usize,

	//total count of words inserted
	word_count: usize
}

impl TfIdf {
	pub fn new() -> TfIdf {
		TfIdf {
			doc_freqs: HashMap::new(),
			term_freqs: HashMap::new(),
			doc_count: 0,
			word_count: 0
		}
	}

	//Add documents to test frequency against
	pub fn add(&mut self, corpus: &str) {
		let tokens = get_tokenized_and_stemmed(corpus);

                let mut seen = HashSet::new();

                tokens.iter()
                    .for_each(|token| {
                        self.term_freqs.entry(token.to_string()).and_modify(|e| {
                            *e += 1;
                        }).or_insert(0);
                        self.word_count += 1;

                        if !seen.contains(&token){
                            seen.insert(token);
                            self.doc_freqs.entry(token.to_string())
                                .and_modify(|e| *e += 1)
                                .or_insert(0);
                        }
                    });

		self.doc_count += 1;
	}

	//Calculate term frequency for one term
	fn tf(&self, term: &str) -> f32 {
		match self.term_freqs.get(term) {
			Some(freq) => *freq as f32 / self.word_count as f32,
			None => 0.0f32
		}
	}

	//Calculate inverse document frequency for one term
	fn idf(&self, term: &str) -> f32 {
		let doc_freq = match self.doc_freqs.get(term) {
			Some(freq) => *freq as f32,
			None => 0.0f32
		};

		let ratio = self.doc_count as f32 / 1.0f32 + doc_freq;

		ratio.ln()
	}

	//Calculate tf-idf for one term
	fn tf_idf(&self, term: &str) -> f32 {
		let tf = self.tf(term);
		let idf = self.idf(term);

		tf * idf
	}

	//Get tf-idf of a string of one or more terms
	pub fn get(&self, terms: &str) -> f32 {
		let tokens = get_tokenized_and_stemmed(terms);
		let token_ref = &tokens;

		let score: f32 = token_ref.iter()
			    .map(|x| self.tf_idf(&x))
			    .fold(0.0f32, |acc, x| acc + x); //add together to later divide by token length to get an average

	  //average the scores
		score / tokens.len() as f32
	}
}


fn get_tokenized_and_stemmed<'a>(text: &'a str) -> Vec<Cow<'a, str>> {
  let en_stemmer = Stemmer::create(Algorithm::English);
  tokenize(text).into_iter()
                .map(|text| en_stemmer.stem(text))
                .collect()
}
