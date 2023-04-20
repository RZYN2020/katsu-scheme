// reference to src/interpreter.rs

use katsu::Katsu;
use regex::Regex;


#[test]
fn test_my_function() {
    let katsu = Katsu::new();
    let tests = include_str!("test.ss");
    // the value after ; in each line is the expected result
    let res = katsu.eval_to_str(tests);
    let re = Regex::new(r";.*>(.*)\n").unwrap();
    for (i, (expected, actual)) in 
    re.captures_iter(tests)
    .map(|c| c.get(1).unwrap().as_str().to_string())
    .zip(res.into_iter().map(|v| v.unwrap()))
    .enumerate() {
        println!("{}: expected: {}, actual: {}", i, expected, actual);
        assert_eq!(expected, actual);
    }
}

