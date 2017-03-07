rs-natural
==========

[![Build Status](https://travis-ci.org/cjqed/rs-natural.svg?branch=master)](https://travis-ci.org/cjqed/rs-natural)

Natural language processing library written in Rust. Still very much a work in progress. Basically an experiment, but hey maybe something cool will come out of it.

Currently working:

* Jaro-Winkler Distance
* Levenshtein Distance
* Tokenizing
* NGrams (with and without padding) 
* Phonetics (Soundex)
* Stemming (Using a fork of [rust-stem](https://github.com/mrordinaire/rust-stem))
* Naive-Bayes classification
* Term Frequency-Inverse Document Frequency(tf-idf)
 
Near-sight goals:

* Logistic regression classification
* Optimize naive-bayes (currently pretty slow)
* Plural/Singular inflector

## How to use ##

Use at your own risk. Some functionality is missing, some other functionality is slow as molasses because it isn't optomized yet. I'm targeting master, and don't offer backward compatibility. 

### Setup ###
It's a crate with a cargo.toml. Add this to your cargo.toml:

```
[dependencies]
natural = "0.2.1"
```

### Distance ###

```rust
extern crate natural;
use natural::distance::jaro_winkler_distance;
use natural::distance::levenshtein_distance;

assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
assert_eq!(jaro_winkler_distance("dixon", "dicksonx"), 0.767); 

```

Note, don't actually `assert_eq!` on JWD since it returns an f64. To test, I actually use:

```rust
fn f64_eq(a: f32, b: f32) {
  assert!((a - b).abs() < 0.01);
}

```

### Phonetics ###

There are two ways to gain access to the SoundEx algorithm in this library, either through a simple `soundex` function that accepts two `&str` parameters and returns a boolean, or through the SoundexWord struct. I will show both here.

```rust
use natural::phonetics::soundex;
use natural::phonetics::SoundexWord;

assert!(soundex("rupert", "robert"));


let s1 = SoundexWord::new("rupert");
let s2 = SoundexWord::new("robert");
assert!(s1.sounds_like(s2));
assert!(s1.sounds_like_str("robert"));

```

### Tokenization ###

```rust
extern crate natural;
use natural::tokenize::tokenize;

assert_eq!(tokenize("hello, world!"), vec!["hello", "world"]);
assert_eq!(tokenize("My dog has fleas."), vec!["My", "dog", "has", "fleas"]);

```

### NGrams ###

You can create an ngram with and without padding, e.g.:

```rust
extern crate natural;

use natural::ngram::get_ngram;
use natural::ngram::get_ngram_with_padding;

assert_eq!(get_ngram("hello my darling", 2), vec![vec!["hello", "my"], vec!["my", "darling"]]);

assert_eq!(get_ngram_with_padding("my fleas", 2, "----"), vec![
  vec!["----", "my"], vec!["my", "fleas"], vec!["fleas", "----"]]);
```

### Classification ###

```rust
extern crate natural;
use natural::classifier::NaiveBayesClassifier;

let mut nbc = NaiveBayesClassifier::new();

nbc.train(STRING_TO_TRAIN, LABEL);
nbc.train(STRING_TO_TRAIN, LABEL);
nbc.train(STRING_TO_TRAIN, LABEL);
nbc.train(STRING_TO_TRAIN, LABEL);

nbc.guess(STRING_TO_GUESS); //returns a label with the highest probability
```

### Tf-Idf ###

```rust
extern crate natural;
use natural::tf_idf::TfIdf;

tf_idf.add("this document is about rust.");
tf_idf.add("this document is about erlang.");
tf_idf.add("this document is about erlang and rust.");
tf_idf.add("this document is about rust. it has rust examples");

println!(tf_idf.get("rust")); //0.2993708f32
println!(tf_idf.get("erlang")); //0.13782766f32

//average of multiple terms
println!(tf_idf.get("rust erlang"); //0.21859923
```