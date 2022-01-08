use std::env;

use rug::{Integer, Complete};

#[derive(Debug)]
struct TestResult {
    continue_testing: bool,
    is_prime: Option<bool>
}


fn test1(n: &Integer) -> TestResult {
    let zero = Integer::from(0);
    let two = Integer::from(2);
    let rem = n%two;
    TestResult {continue_testing: rem==zero, is_prime: Some(rem==zero)}
}

fn test2(n: &Integer) -> TestResult {
    let zero = Integer::from(0);
    let two = Integer::from(2);
    let rem = n%two;
    TestResult {continue_testing: rem!=zero, is_prime: Some(rem!=zero)}
}

fn is_prime(n: &Integer) -> bool{

    let tests = [test1, test2];
    let mut i = 1;
    let mut result = tests[0](n);

    while result.continue_testing && i < tests.len() {
        result = tests[i](n);
        i += 1;
    }
    
    result.is_prime.unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let input = get_input(args);
    
    if let Ok(incomplete) = Integer::parse(&input) {
        let n = incomplete.complete();
        let output = if is_prime(&n) {"is"} else {"is not"};
        println!("{0} {1} prime",n, output);
    }else{
        panic!("Error parsing the input {}", input);
    }
}

fn get_input(args: Vec<String>) -> String {
    let default_int = "10";
    let input = if args.len() < 0x2 { default_int }else{ &args[1] };
    String::from(input)
}
