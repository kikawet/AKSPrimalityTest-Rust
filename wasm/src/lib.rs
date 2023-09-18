use aks_primality_test::primality_check::{self, TestInput};
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn is_prime(n: &str) -> Option<bool> {
    Some(primality_check::is_prime(&TestInput::from_str(n).ok()?))
}
