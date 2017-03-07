extern crate natural;

use natural::stem;
use std::ops::Deref;

pub static INPUT: &'static str  = include_str!("data/voc.txt");
pub static RESULT: &'static str = include_str!("data/output.txt");

fn test_loop<
    's,
    I0: Iterator<Item = T>,
    I1: Iterator<Item = T>,
    T: Deref<Target = str>>(
    tests: I0,
    results: I1
) {
    for (test, expect) in tests.zip(results) {
        let test = test.trim_right();
        let expect = expect.trim_right();
        let stemmed = stem::get(test.trim_right());

        assert_eq!(stemmed.trim_right(), expect);
    }
}

#[test]
fn lexicon() {
    let input_s = INPUT.split('\n');
    let result_s = RESULT.split('\n');

    test_loop(input_s, result_s);
}