#[macro_use]
extern crate log;

use std::{
    env,
};

use rug::{
    Complete, Integer,
};

mod aks;
use aks::primality_check::is_prime;

fn main() {
    env_logger::init();
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