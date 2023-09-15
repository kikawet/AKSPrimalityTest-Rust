use rug::Integer;

use crate::primality_check::primality_check_rug::{test1, test2, test3, test4, test5, test6};

mod primality_check_rug;

#[derive(Debug)]
pub(crate) struct TestResult {
    is_prime: Option<bool>,
}

pub(crate) struct Context {
    r: u128,
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
