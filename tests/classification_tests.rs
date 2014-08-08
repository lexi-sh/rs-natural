extern crate natural;

use natural::classifier::NaiveBayesClassifier;

#[test]
fn test_basic_usage() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str("Debug derive traits into impls that I use with my rust code"), String::from_str("rust"));
  nbc.train(String::from_str("deriving show for your impl can definitely help you debug your rust code"), String::from_str("rust"));
  nbc.train(String::from_str("Use more metaprogramming when writing ruby"), String::from_str("ruby"));
  nbc.train(String::from_str("Classes can have either instance variables or class variables"), String::from_str("ruby"));
  assert_eq!(nbc.guess(String::from_str("This class is about ruby")), String::from_str("ruby"));
  assert_eq!(nbc.guess(String::from_str("debug this rust code")), String::from_str("rust"));
  
  nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str("i am long qqqq"), String::from_str("buy"));
  nbc.train(String::from_str("buy the q''s"), String::from_str("buy"));
  nbc.train(String::from_str("short gold"), String::from_str("sell"));
  nbc.train(String::from_str("sell gold"), String::from_str("sell"));
  assert_eq!(nbc.guess(String::from_str("i am short silver")), String::from_str("sell"));
  assert_eq!(nbc.guess(String::from_str("i am long copper")), String::from_str("buy"));
}

#[test]
fn no_fail_on_empty_strings() {
  let mut nbc = NaiveBayesClassifier::new();
  nbc.train(String::from_str(""), String::from_str(""));
  assert!(true);
}