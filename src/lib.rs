#[cfg(feature = "serde_support")]
extern crate serde;

extern crate rust_stemmers;

pub mod distance;
pub mod ngram;
pub mod tokenize;
pub mod phonetics;
pub mod classifier;
pub mod tf_idf;
