use std::{
    env,
    ops::{Add, AddAssign, Not, Range, Sub, SubAssign},
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rug::{
    ops::{CompleteRound, Pow},
    Assign, Complete, Float, Integer,
};

#[derive(Debug)]
struct TestResult {
    continue_testing: bool,
    is_prime: Option<bool>,
}

struct Context {
    r: u128,
}

/// * @brief Step 1 - If n = a^b for integers a > 1 and b > 1, output composite
fn test1(n: &Integer, _: &mut Context) -> TestResult {
    let n_as_float = Float::with_val(u32::MAX, n);
    let top_limit = calculate_log2(n);

    let found_any_integer = (2..=top_limit).into_par_iter().any(|b| -> bool {
        n_as_float
            .as_ref()
            .as_float()
            .pow(1f64 / (b as f64))
            .complete(256)
            .is_integer()
    });

    print!("Test 1 done");

    if found_any_integer.not() {
        println!();
        TestResult {
            continue_testing: true,
            is_prime: None,
        }
    } else {
        println!(" with fail");
        TestResult {
            continue_testing: false,
            is_prime: Some(false),
        }
    }
}

/// * @brief Step 2 - Find the smallest r such that Or(n) > (log2 n)^2
fn test2(n: &Integer, context: &mut Context) -> TestResult {
    let maxk = Integer::from(calculate_log2(n)).pow(2);
    let maxr = Integer::from(calculate_log2(n))
        .pow(5)
        .add(Integer::from(1u8))
        .max(Integer::from(3u8));

    let mut next_r = true;
    let mut r = Integer::from(2);

    while next_r && r.lt(&maxr) {
        next_r = false;

        let mut k = Integer::from(1);
        while next_r.not() && k.le(&maxk) {
            if let Some(modd) = n.pow_mod_ref(&k, &r) {
                let modulo = Integer::from(modd);
                next_r = modulo.eq(&1u8) || modulo.eq(&0u8);
            }
            k.add_assign(1);
        }

        r.add_assign(1);
    }

    r.sub_assign(1);

    println!("r={}", &r);

    context.r = r.try_into().unwrap();

    TestResult {
        continue_testing: true,
        is_prime: None,
    }
}

/// * @brief Step 3 - If 1 < gcd(a,n) < n for some a ≤ r, output composite
fn test3(n: &Integer, context: &mut Context) -> TestResult {
    let found_any = (context.r..1)
        .into_par_iter()
        .map(|i| -> Integer { Integer::from(i) })
        .map(|x| -> Integer { n.gcd_ref(&x).complete() })
        .any(|gcd| -> bool { 1 < gcd || gcd < *n });

    print!("Test 3 done");

    if found_any {
        println!();
        TestResult {
            continue_testing: false,
            is_prime: Some(false),
        }
    } else {
        println!(" with fail");
        TestResult {
            continue_testing: true,
            is_prime: None,
        }
    }
}

/// * @brief Step 6 - n must be prime
fn test6(_: &Integer, _: &mut Context) -> TestResult {
    TestResult {
        continue_testing: false,
        is_prime: Some(true),
    }
}

fn is_prime(n: &Integer) -> bool {
    let tests = [test1, test2, test3, test6];
    let mut i = 1;
    let mut context = Context { r: 0 };
    let mut result = tests[0](n, &mut context);

    while result.continue_testing && i < tests.len() {
        result = tests[i](n, &mut context);
        i += 1;
    }

    result.is_prime.unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = get_input(args);

    if let Ok(incomplete) = Integer::parse(&input) {
        let n = incomplete.complete();
        let output = if is_prime(&n) { "is" } else { "is not" };
        println!("{0} {1} prime", n, output);
    } else {
        panic!("Error parsing the input {}", input);
    }
}

fn get_input(args: Vec<String>) -> String {
    let default_int = "31";
    let input = if args.len() < 0x2 {
        default_int
    } else {
        &args[1]
    };
    String::from(input)
}

fn calculate_log2(n: &Integer) -> u32 {
    n.significant_bits() - 1u32
}
