# AKSPrimalityTest-Rust

## What is My Project?

Proof of concept programming AKS primality test in rust using vscode with remote code using a docker container

# Run
List of primes numbers - https://primes.utm.edu/lists/small/small.html

```
cargo run --release -- <CANDIDATE>
```

# Tests
```
cargo tests --release -- --tests-threads 3
```

# Version
- cargo 1.56.0
- rustc 1.56.1

# DONE
 - [x] read user input
 - [x] create base structure
 - [x] import into docker
 - [x] add tests
   - [x] test 1 - If n = a^b for integers a > 1 and b > 1, output composite.
   - [x] step 2 - Find the smallest r such that Or(n) > (log2 n)^2
   - [x] test 3 - If 1 < gcd(a,n) < n for some a ≤ r, output composite
   - [x] test 4 - If n ≤ r, output prime
   - [x] test 5 - check that for every coeficient (ai) in (x-1)^n ai%n == 0 // Calculate binomials like madman
 - [x] add tests to the fn is_prime
 - [x] make sure tests actually pass 😒
 - [x] refactor into multiple modules
 - [x] profile -> https://github.com/svenstaro/cargo-profiler