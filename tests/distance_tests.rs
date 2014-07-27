extern crate natural;

use natural::distance::jaro_winkler_distance;

#[test]
fn test_jaro_winkler() {
  f64_eq(jaro_winkler_distance("one", "one"), 1.0);
  f64_eq(jaro_winkler_distance("one", "one"), 1f64);
  f64_eq(jaro_winkler_distance("one", ""), 0f64);
  f64_eq(jaro_winkler_distance("", ""), 0f64);
  f64_eq(jaro_winkler_distance("", "one"), 0f64);
  f64_eq(jaro_winkler_distance("dixon", "dicksonx"), 0.746666f64);  
}

fn f64_eq(a: f64, b: f64) {
  assert!((a - b).abs() < 0.01);
}