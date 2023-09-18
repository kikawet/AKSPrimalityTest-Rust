use itertools::iproduct;
#[cfg(feature = "log")]
use log::debug;
#[cfg(feature = "log")]
use log::trace;
use malachite::{
    num::{
        arithmetic::traits::{CheckedRoot, Gcd, Mod, ModPow},
        basic::traits::{One, Two},
        logic::traits::SignificantBits,
    },
    Natural,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::ops::Add;
#[cfg(feature = "log")]
use std::time::Instant;

use super::{Context, TestResult};

/// * @brief Step 1 - If n = a^b for integers a > 1 and b > 1, output composite
pub(crate) fn test1(n: &Natural, _: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let top_limit = n.significant_bits() - 1;

    let found_any_integer = (2..=top_limit)
        .into_par_iter()
        .any(|b| n.clone().checked_root(b).is_some());

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
pub(crate) fn test2(n: &Natural, context: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let n_log2 = n.significant_bits() - 1;
    let max_k = n_log2.pow(2);
    let max_r = n_log2.pow(5).add(1).max(3);

    let k_range = (1..=max_k).map(Natural::from);
    let r_range = (2..max_r).map(Natural::from);

    let final_r = iproduct!(r_range, k_range)
        .find(|(r, k)| (n.mod_op(r)).mod_pow(k, r) > Natural::ONE)
        .and_then(|(r, _)| r.limbs().next())
        .unwrap_or(max_r);

    #[cfg(feature = "log")]
    {
        debug!("Step 2 done \telapsed time={:?}", start.elapsed());
        trace!("\tr={}", &final_r);
    }

    context.r = final_r.into();

    TestResult { is_prime: None }
}

/// * @brief Step 3 - If 1 < gcd(a,n) < n for some a ≤ r, output composite
pub(crate) fn test3(n: &Natural, context: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let found_any = (1..context.r)
        .into_par_iter()
        .map(Natural::from)
        .map(|x| n.gcd(x))
        .any(|gcd| 1 < gcd && gcd < *n);

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
pub(crate) fn test4(n: &Natural, context: &mut Context) -> TestResult {
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
pub(crate) fn test5(n: &Natural, _: &mut Context) -> TestResult {
    #[cfg(feature = "log")]
    let start = Instant::now();

    let limit = (n / Natural::TWO) - Natural::ONE;
    let mut current_root = Natural::ONE;
    let mut i = Natural::ONE;

    // Calculate binomials in an iterative way
    let has_divisible_coefficient = loop {
        if i >= limit {
            break false;
        }

        #[cfg(feature = "log")]
        if &i % Natural::from(100_000u32) == 0 {
            trace!("Progres: {:?}", (&i / &limit));
        }

        current_root *= (n - &i) + Natural::ONE;
        current_root /= &i;

        if (&current_root % n) != 0 {
            break true;
        }

        i += Natural::ONE;
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
pub(crate) fn test6(_: &Natural, _: &mut Context) -> TestResult {
    TestResult {
        is_prime: Some(true),
    }
}
