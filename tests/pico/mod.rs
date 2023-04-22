// reference to src/interpreter.rs

use core::prelude;

use katsu::Katsu;
use regex::Regex;

macro_rules! cap_to_string {
    ($expr:expr, $i:expr) => {
        $expr.get($i).unwrap().as_str().to_string()
    };
}

#[test]
fn run_all_test() {
    let katsu = Katsu::new();
    let tests = include_str!("test.ss");
    // the value after ; in each line is the expected result
    let res = katsu.eval_all(tests);
    let re = Regex::new(r"(.*);.*>(.*)\n").unwrap();
    for (i, ((expr, expected), actual)) in 
    re.captures_iter(tests)
    .map(|c| (cap_to_string!(c, 1), cap_to_string!(c, 2)))
    .zip(res)
    .enumerate() {
        print!("epxr{}:{} => expected: {}, actual: {}", i, expr, expected, actual);
        assert_eq!(expected, actual);
        println!("    \x1b[32mtest {} pass!\x1b[0m", i);
    }
}

#[test]
fn test() {
    let katsu = Katsu::new();
    let prelude = "(define list (lambda l l))";
    let test = "(list 1 2 3)";
    katsu.eval(prelude);
    let res = katsu.eval_to_str(test);
    println!("{}", res);
}
