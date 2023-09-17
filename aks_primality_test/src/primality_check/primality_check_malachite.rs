#[cfg(feature = "log")]
use log::debug;
use malachite::{
    num::{arithmetic::traits::CheckedRoot, logic::traits::SignificantBits},
    Natural,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
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

/// * @brief Step 6 - n must be prime
pub(crate) fn test6(_: &Natural, _: &mut Context) -> TestResult {
    TestResult {
        is_prime: Some(true),
    }
}
