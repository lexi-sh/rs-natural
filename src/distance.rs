struct calc_objects<'a> {
  first: &'a str,
  second: &'a str
}
pub fn jaro_winkler_distance<'sl>(str1: &'sl str, str2: &'sl str) -> f64 {
  let co = calc_objects{ first: str1, second: str2 };
  let m = get_num_matching_chars(&co);
  let t = get_num_transpositions(&co) / 2.0;
  0.0
}

fn get_num_matching_chars(obj: &calc_objects) -> f64 {
  0.0
}

fn get_num_transpositions(obj: &calc_objects) -> f64 {
  0.0
}
