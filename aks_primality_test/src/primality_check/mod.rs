#[cfg(any(test, not(feature = "exclude_wasm")))]
mod primality_check_malachite;
#[cfg(not(feature = "exclude_wasm"))]
use malachite::Natural;

#[cfg(feature = "exclude_wasm")]
use rug::Integer;
#[cfg(any(test, feature = "exclude_wasm"))]
mod primality_check_rug;
#[cfg(feature = "exclude_wasm")]
use crate::primality_check::primality_check_rug::{test1, test2, test3, test4, test5, test6};

#[derive(Debug)]
pub(crate) struct TestResult {
    is_prime: Option<bool>,
}

pub(crate) struct Context {
    r: u128,
}

#[cfg(feature = "exclude_wasm")]
pub type TestInput = Integer;
#[cfg(not(feature = "exclude_wasm"))]
pub type TestInput = Natural;

#[must_use]
pub fn is_prime(n: &TestInput) -> bool {
    #[cfg(feature = "exclude_wasm")]
    let tests = [test1, test2, test3, test4, test5, test6];
    #[cfg(not(feature = "exclude_wasm"))]
    let tests = [
        primality_check_malachite::test1,
        primality_check_malachite::test2,
        primality_check_malachite::test3,
        primality_check_malachite::test4,
        primality_check_malachite::test5,
        primality_check_malachite::test6,
    ];
    let mut context = Context { r: 0 };

    for test in tests {
        let result = test(n, &mut context);
        if let Some(is_prime) = result.is_prime {
            return is_prime;
        }
    }

    unreachable!("By this point the test must be finished")
}

#[cfg(all(test))]
mod malachite_tests {
    use malachite::Natural;

    use crate::primality_check::primality_check_rug;
    use rug::Integer;

    use crate::primality_check::primality_check_malachite;

    use super::Context;

    fn mock_context() -> Context {
        Context { r: 0 }
    }

    #[test]
    fn test1_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                primality_check_malachite::test1(&candidate.1, &mut mock_context()).is_prime
                    != primality_check_rug::test1(&candidate.0, &mut mock_context()).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }

    #[test]
    fn test2_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                let mut context_mal = mock_context();
                let mut context_rug = mock_context();
                primality_check_malachite::test2(&candidate.1, &mut context_mal);
                primality_check_rug::test2(&candidate.0, &mut context_rug);

                context_mal.r != context_rug.r
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }

    #[test]
    fn test3_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                let mut context_mal = mock_context();
                let mut context_rug = mock_context();
                primality_check_malachite::test2(&candidate.1, &mut context_mal);
                primality_check_rug::test2(&candidate.0, &mut context_rug);

                primality_check_malachite::test3(&candidate.1, &mut context_mal).is_prime
                    != primality_check_rug::test3(&candidate.0, &mut context_rug).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }

    #[test]
    fn test4_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                let mut context_mal = mock_context();
                let mut context_rug = mock_context();
                primality_check_malachite::test2(&candidate.1, &mut context_mal);
                primality_check_rug::test2(&candidate.0, &mut context_rug);

                primality_check_malachite::test4(&candidate.1, &mut context_mal).is_prime
                    != primality_check_rug::test4(&candidate.0, &mut context_rug).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }

    #[test]
    fn test5_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                primality_check_malachite::test5(&candidate.1, &mut mock_context()).is_prime
                    != primality_check_rug::test5(&candidate.0, &mut mock_context()).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }

    #[test]
    fn test6_works() {
        let failed = (2..=1000)
            .map(|i: usize| -> (Integer, Natural) { (Integer::from(i), Natural::from(i)) })
            .find(|candidate| {
                primality_check_malachite::test6(&candidate.1, &mut mock_context()).is_prime
                    != primality_check_rug::test6(&candidate.0, &mut mock_context()).is_prime
            })
            .map(|failed| failed.0);

        assert_eq!(failed, None);
    }
}
