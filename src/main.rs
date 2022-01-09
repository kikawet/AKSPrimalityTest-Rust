use std::env;

use rayon::iter::{ParallelIterator, IntoParallelIterator};
use rug::{Integer, Complete, Float, ops::{Pow, CompleteRound}};

#[derive(Debug)]
struct TestResult {
    continue_testing: bool,
    is_prime: Option<bool>
}

/// * @brief Step 1 - If n = a^b for integers a > 1 and b > 1, output composite
fn test1(n: &Integer) -> TestResult {
    let n_as_float = Float::with_val(u32::MAX, n);
    let top_limit = n.significant_bits() - 1; // log2(n)
    
    let found_any_integer = (2..=top_limit).into_par_iter().find_any(|b| -> bool {
        n_as_float.as_ref()
            .as_float()
            .pow(1f32/(*b as f32))
            .complete(20)
            .is_integer()        
    });

    if found_any_integer == None {
        TestResult {continue_testing: true, is_prime: None}
    }else{
        TestResult {continue_testing: false, is_prime: Some(false)}
    }
}

fn test2(n: &Integer) -> TestResult {
    let zero = Integer::from(0);
    let two = Integer::from(2);
    let rem = n%two;
    TestResult {continue_testing: rem!=zero, is_prime: Some(rem!=zero)}
}

fn is_prime(n: &Integer) -> bool{

    let tests = [test1];
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
    let default_int = "31";
    let input = if args.len() < 0x2 { default_int }else{ &args[1] };
    String::from(input)
}
