use std::env;

use aks_primality_test::primality_check::is_prime;
use env_logger::Env;
use log::{info, LevelFilter};
use rug::{Complete, Integer};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or(LevelFilter::Info.as_str()))
        .init();

    let default_input = "31";
    let input = env::args().nth(1).unwrap_or(default_input.to_string());

    if let Ok(incomplete) = Integer::parse(&input) {
        let n = incomplete.complete();
        let not = if is_prime(&n) { "" } else { " not" };
        info!("{n} is{not} prime");
    } else {
        panic!("Error parsing the input {input}");
    }
}
