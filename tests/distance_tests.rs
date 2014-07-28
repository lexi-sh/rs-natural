extern crate natural;

use natural::distance::jaro_winkler_distance;

#[test]
fn test_jaro_winkler() {
  f64_eq(jaro_winkler_distance("one", "one"), 1f32);
  f64_eq(jaro_winkler_distance("one", ""), 0f32);
  f64_eq(jaro_winkler_distance("", ""), 0f32);
  f64_eq(jaro_winkler_distance("", "one"), 0f32);
  println!("{}", jaro_winkler_distance("DWAYNE", "DUANE"));
  f64_eq(jaro_winkler_distance("DWAYNE", "DUANE"), 0.822);  
}

fn f64_eq(a: f32, b: f32) {
  assert!((a - b).abs() < 0.03);
}