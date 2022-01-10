use std::{
    env,
    ops::{Add, Not, Sub},
    time::Instant,
};

use log::{debug, error, info};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rug::{
    ops::{CompleteRound, Pow},
    Complete, Float, Integer,
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
    let start = Instant::now();

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

    let duration = start.elapsed();
    debug!("Test 1 done \telapsed time={:?}", duration);

    if found_any_integer.not() {
        TestResult {
            continue_testing: true,
            is_prime: None,
        }
    } else {
        debug!("\t test didn't pass");
        TestResult {
            continue_testing: false,
            is_prime: Some(false),
        }
    }
}

/// * @brief Step 2 - Find the smallest r such that Or(n) > (log2 n)^2
fn test2(n: &Integer, context: &mut Context) -> TestResult {
    let start = Instant::now();

    let maxk: u128 = Integer::from(calculate_log2(n)).pow(2).try_into().unwrap();
    let maxr: u128 = Integer::from(calculate_log2(n))
        .pow(5)
        .add(Integer::from(1u8))
        .max(Integer::from(3u8))
        .try_into()
        .unwrap();

    let k_range = (1..=maxk)
        .into_par_iter()
        .map(Integer::from)
        .into_par_iter();

    let final_r = (2..maxr)
        .into_par_iter()
        .find_first(|r| -> bool {
            let r_as_ref_integer = &Integer::from(*r);
            let next_r = k_range
                // @TODO: make sure this only clones the iterator
                .clone() // hopefully this only clones the iterator :)
                .any(|k| -> bool {
                    if let Some(modd) = n.pow_mod_ref(&k, r_as_ref_integer) {
                        let modulo = Integer::from(modd);
                        return modulo.eq(&1u8) || modulo.eq(&0u8);
                    }
                    false
                });

            next_r.not()
        })
        .unwrap_or(maxr);

    let duration = start.elapsed();
    debug!("Step 2 done \telapsed time={:?}", duration);
    debug!("\tr={}", &final_r);

    context.r = final_r;

    TestResult {
        continue_testing: true,
        is_prime: None,
    }
}

/// * @brief Step 3 - If 1 < gcd(a,n) < n for some a ≤ r, output composite
fn test3(n: &Integer, context: &mut Context) -> TestResult {
    let start = Instant::now();

    let found_any = (1..context.r)
        .into_par_iter()
        .map(Integer::from )
        .map(|x| -> Integer { n.gcd_ref(&x).complete() })
        .any(|gcd| -> bool { 1 < gcd && gcd < *n });

    let duration = start.elapsed();
    debug!("Test 3 done \telapsed time={:?}", duration);

    if found_any {
        debug!("\t test didn't pass");
        TestResult {
            continue_testing: false,
            is_prime: Some(false),
        }
    } else {
        TestResult {
            continue_testing: true,
            is_prime: None,
        }
    }
}

/// * @brief Step 4 - If n ≤ r, output prime
fn test4(n: &Integer, context: &mut Context) -> TestResult {
    let start = Instant::now();

    let is_le = n <= &context.r;

    let duration = start.elapsed();
    debug!("Test 4 done \telapsed time={:?}", duration);

    if is_le {
        debug!("\t test didn't pass");
        TestResult {
            continue_testing: false,
            is_prime: Some(true),
        }
    } else {
        TestResult {
            continue_testing: true,
            is_prime: None,
        }
    }
}

/// * @brief Step 5 - if any coeficient (ai) in (x-1)^n ai%n != 0, output composite
fn test5(n: &Integer, _: &mut Context) -> TestResult {
    let start = Instant::now();

    let one = Integer::from(1);
    let limit = n / Integer::from(2) - &one;
    let mut current_root = one.clone();
    let mut i = one.clone();

    // Calculate binomials in an iterative way
    let has_divisible_coefficient = loop {
        if i >= limit {
            break false;
        }

        current_root *= n.sub(&i).complete().add(&one);
        current_root /= &i;

        if !current_root.is_divisible(n) {
            break true;
        }

        i += &one;
    };

    let duration = start.elapsed();
    debug!("Test 5 done \telapsed time={:?}", duration);

    if has_divisible_coefficient {
        debug!("\t test didn't pass");
        TestResult {
            continue_testing: false,
            is_prime: Some(false),
        }
    } else {
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
    let tests = [test1, test2, test3, test4, test5, test6];
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
        info!("{0} {1} prime", n, output);
    } else {
        error!("Error parsing the input {}", input);
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


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_nums_below_1000() {
        let primes = HashSet::from([2,   3,   5,   7,  11,  13,  17,  19,  23,  29,  31,  37,  41, 43,  47,  53,  59,  61,  67,  71,  73,  79,  83,  89,  97, 101,
            103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239,
            241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
            401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569,
            571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733,
            739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911,
            919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997]);

        assert!((2..=1000).into_par_iter().all(|candidate| -> bool {
            primes.contains(&candidate) == is_prime(&Integer::from(candidate))
        }));
    }
}