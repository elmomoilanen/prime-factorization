//! Integer factorization command line program.
//!
//! Natural number N, given in range [2, 2^128 - 1], is decomposed into a product
//! of its prime factors, p_1^k_1 * p_2^k_2 * ... * p_m^k_m, in which each p_ term
//! represents a prime factor and the count of its occurrence is marked by the
//! corresponding k_ term.
//!
//! This decomposition is of huge importance as based on the fundamental theorem
//! of arithmetic every natural number larger than one is either a prime itself
//! or can be represented as a product of primes that is unique up to the order
//! of these prime numbers.
//!
//! E.g., a natural number 30 has the prime factor representation 2*3*5. Given this
//! number 30 as an input, this program would return the prime factors in the following manner
//!
//! ```bash
//! ./target/release/prime_factorization 30
//! factors: 2, 3, 5
//! ```
//!
//! or for the input 50, which prime factor representation is 2*5*5, the result would be
//!
//! ```bash
//! ./target/release/prime_factorization 50
//! factors: 2, 5, 5
//! ```
//!
//! which indicates that the returned output (prime factors) contains all terms to restore
//! the original input number (as a product of these factors), not just the unique factors.
//!
//! If the input number is a prime number, the returned factors would of course contain
//! only the passed number as the following example shows
//!
//! ```bash
//! ./target/release/prime_factorization 17
//! factors: 17
//! ```
//!
//! Thus, this program can also be used as a primality tester.
//!
//! For the last example, the largest 128 bit number 2^128 - 1 has the following
//! prime factor representation
//!
//! ```bash
//! ./target/release/prime_factorization 340282366920938463463374607431768211455
//! factors: 3, 5, 17, 257, 641, 65537, 274177, 6700417, 67280421310721
//! ```
//!
use std::{env, process};

extern crate num;
extern crate rand;

mod arith;
mod factorization;
mod ladder_bytes;
mod parser;
mod prime;
mod small_primes;

use factorization::{Factorization, UInt};

fn main() {
    let args: Vec<String> = env::args().collect();

    let num: u128 = parser::parse_arguments(&args[1..]).unwrap_or_else(|err| {
        if err == "help" {
            process::exit(0);
        }
        eprintln!("Error with command line args: {}", err);
        process::exit(1);
    });

    if num <= u32::MAX as u128 {
        factorize(num as u32);
    } else if num <= u64::MAX as u128 {
        factorize(num as u64);
    } else {
        factorize(num);
    }
}

fn factorize<T: 'static + UInt>(num: T) {
    let mut factors = Factorization::new(num);
    factors.run();

    println!("{}", factors);
}
