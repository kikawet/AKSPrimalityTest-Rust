//#[cfg(not(feature = "rug"))]
mod primality_check_malachite;
#[cfg(not(feature = "rug"))]
use malachite::Natural;

#[cfg(feature = "rug")]
use rug::Integer;
#[cfg(feature = "rug")]
mod primality_check_rug;
#[cfg(feature = "rug")]
use crate::primality_check::primality_check_rug::{test1, test2, test3, test4, test5, test6};

#[derive(Debug)]
pub(crate) struct TestResult {
    is_prime: Option<bool>,
}

pub(crate) struct Context {
    r: u128,
}

#[cfg(feature = "rug")]
pub type TestInput = Integer;
#[cfg(not(feature = "rug"))]
pub type TestInput = Natural;

#[must_use]
pub fn is_prime(n: &TestInput) -> bool {
    #[cfg(feature = "rug")]
    let tests = [test1, test2, test3, test4, test5, test6];
    #[cfg(not(feature = "rug"))]
    let tests = [
        primality_check_malachite::test1,
        primality_check_malachite::test6,
    ]; //, test2, test3, test4, test5, test6];
    let mut context = Context { r: 0 };

    for test in tests {
        let result = test(n, &mut context);
        if let Some(is_prime) = result.is_prime {
            return is_prime;
        }
    }

    unreachable!("By this point the test must be finished")
}

#[cfg(all(test, feature = "rug"))]
mod malachite_tests {

    use std::collections::HashSet;

    use malachite::Natural;

    #[cfg(feature = "rug")]
    use crate::primality_check::primality_check_rug;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    #[cfg(feature = "rug")]
    use rug::Integer;

    use crate::primality_check::primality_check_malachite;

    use super::Context;

    //use crate::primality_check::{primality_check_malachite, primality_check_rug, Context};

    fn primes_below1000() -> HashSet<usize> {
        HashSet::from([
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ])
    }

    fn mock_context() -> Context {
        Context { r: 0 }
    }

    #[test]
    fn test1_works() {
        let failed = (2..=1000)
            .into_par_iter()
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find_any(|candidate| {
                primality_check_malachite::test1(&candidate.1, &mut mock_context()).is_prime
                    != primality_check_rug::test1(&candidate.0, &mut mock_context()).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }
}
