use std::{
    ops::{Add, Not, Sub},
    time::Instant,
};

use log::{debug, trace};

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

    debug!("Test 1 done \telapsed time={:?}", start.elapsed());

    if found_any_integer {
        debug!("\t test didn't pass");
    }

    TestResult {
        continue_testing: !found_any_integer,
        is_prime: found_any_integer.then_some(false),
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

    debug!("Step 2 done \telapsed time={:?}", start.elapsed());
    trace!("\tr={}", &final_r);

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
        .map(Integer::from)
        .map(|x| -> Integer { n.gcd_ref(&x).complete() })
        .any(|gcd| -> bool { 1 < gcd && gcd < *n });

    debug!("Test 3 done \telapsed time={:?}", start.elapsed());

    if found_any {
        debug!("\t test didn't pass");
    }

    TestResult {
        continue_testing: !found_any,
        is_prime: found_any.then_some(false),
    }
}

/// * @brief Step 4 - If n ≤ r, output prime
fn test4(n: &Integer, context: &mut Context) -> TestResult {
    let start = Instant::now();

    let is_le = n <= &context.r;

    debug!("Test 4 done \telapsed time={:?}", start.elapsed());

    if is_le {
        debug!("\t test didn't pass");
    }

    TestResult {
        continue_testing: !is_le,
        is_prime: is_le.then_some(true),
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

    debug!("Test 5 done \telapsed time={:?}", start.elapsed());

    if has_divisible_coefficient {
        debug!("\t test didn't pass");
    }

    TestResult {
        continue_testing: !has_divisible_coefficient,
        is_prime: has_divisible_coefficient.then_some(false),
    }
}

/// * @brief Step 6 - n must be prime
fn test6(_: &Integer, _: &mut Context) -> TestResult {
    TestResult {
        continue_testing: false,
        is_prime: Some(true),
    }
}

pub fn is_prime(n: &Integer) -> bool {
    let tests = [test1, test2, test3, test4, test5, test6];
    let mut context = Context { r: 0 };

    for test in tests {
        let result = test(n, &mut context);
        if !result.continue_testing {
            return result.is_prime.unwrap();
        }
    }

    unreachable!("By this point the test must be finished")
}

fn calculate_log2(n: &Integer) -> u32 {
    n.significant_bits() - 1u32
}
