use std::ops::{Add, AddAssign, DivAssign, MulAssign, Sub};

use itertools::iproduct;

#[cfg(feature = "log")]
use log::{debug, trace};
#[cfg(feature = "log")]
use std::time::Instant;

use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    prelude::ParallelBridge,
};

use rug::{ops::Pow, Complete, Float, Integer};

#[derive(Debug)]
struct TestResult {
    is_prime: Option<bool>,
}

struct Context {
    r: u128,
}

/// * @brief Step 1 - If n = a^b for integers a > 1 and b > 1, output composite
fn test1(n: &Integer, _: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let top_limit = calculate_log2(n);
    let n: Float = Float::with_val(256, 0) + n;

    let found_any_integer = (2..=top_limit)
        .into_par_iter()
        .map(f64::from)
        .any(|b| n.clone().pow(1f64 / b).is_integer());

    #[cfg(feature = "log")]
    {
        debug!("Test 1 done \telapsed time={:?}", start.elapsed());

        if found_any_integer {
            debug!("\t test didn't pass");
        }
    }

    TestResult {
        is_prime: found_any_integer.then_some(false),
    }
}

/// * @brief Step 2 - Find the smallest r such that Or(n) > (log2 n)^2
fn test2(n: &Integer, context: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let max_k: u128 = Into::<u128>::into(calculate_log2(n)).pow(2);
    let max_r: u128 = Into::<u128>::into(calculate_log2(n)).pow(5).add(1).max(3);

    let k_range = (1..=max_k).map(Integer::from);
    let r_range = (2..max_r).map(Integer::from);

    let final_r = iproduct!(r_range, k_range)
        .par_bridge()
        .find_first(|(r, k)| {
            n.pow_mod_ref(k, r)
                .map(Integer::from)
                .filter(|modulo| modulo.eq(&1u8) || modulo.eq(&0u8))
                .map_or(true, |_| false)
        })
        .map_or(max_r, |(r, _)| {
            r.to_u128().expect("Unable to finish step 2")
        });

    #[cfg(feature = "log")]
    {
        debug!("Step 2 done \telapsed time={:?}", start.elapsed());
        trace!("\tr={}", &final_r);
    }

    context.r = final_r;

    TestResult { is_prime: None }
}

/// * @brief Step 3 - If 1 < gcd(a,n) < n for some a ≤ r, output composite
fn test3(n: &Integer, context: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let found_any = (1..context.r)
        .into_par_iter()
        .map(Integer::from)
        .map(|x| -> Integer { n.gcd_ref(&x).complete() })
        .any(|gcd| -> bool { 1 < gcd && gcd < *n });

    #[cfg(feature = "log")]
    {
        debug!("Test 3 done \telapsed time={:?}", start.elapsed());

        if found_any {
            debug!("\t test didn't pass");
        }
    }

    TestResult {
        is_prime: found_any.then_some(false),
    }
}

/// * @brief Step 4 - If n ≤ r, output prime
fn test4(n: &Integer, context: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let is_le = n <= &context.r;

    #[cfg(feature = "log")]
    {
        debug!("Test 4 done \telapsed time={:?}", start.elapsed());

        if is_le {
            debug!("\t test didn't pass");
        }
    }

    TestResult {
        is_prime: is_le.then_some(true),
    }
}

/// * @brief Step 5 - if any coeficient (ai) in (x-1)^n ai%n != 0, output composite
fn test5(n: &Integer, _: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let one = Integer::from(1);
    let limit = (n / Integer::from(2)) - &one;
    let mut current_root = one.clone();
    let mut i = one.clone();

    // Calculate binomials in an iterative way
    let has_divisible_coefficient = loop {
        if i >= limit {
            break false;
        }

        #[cfg(feature = "log")]
        if i.is_divisible(&Integer::from(100_000)) {
            trace!("Progres: {:?}", (&i / &limit).complete());
        }

        current_root.mul_assign(n.sub(&i).complete().add(&one));
        current_root.div_assign(&i);

        if !current_root.is_divisible(n) {
            break true;
        }

        i.add_assign(&one);
    };

    #[cfg(feature = "log")]
    {
        debug!("Test 5 done \telapsed time={:?}", start.elapsed());

        if has_divisible_coefficient {
            debug!("\t test didn't pass");
        }
    }

    TestResult {
        is_prime: has_divisible_coefficient.then_some(false),
    }
}

/// * @brief Step 6 - n must be prime
fn test6(_: &Integer, _: &mut Context) -> TestResult {
    TestResult {
        is_prime: Some(true),
    }
}

#[must_use]
pub fn is_prime(n: &Integer) -> bool {
    let tests = [test1, test2, test3, test4, test5, test6];
    let mut context = Context { r: 0 };

    for test in tests {
        let result = test(n, &mut context);
        if let Some(is_prime) = result.is_prime {
            return is_prime;
        }
    }

    unreachable!("By this point the test must be finished")
}

fn calculate_log2(n: &Integer) -> u32 {
    n.significant_bits() - 1u32
}
