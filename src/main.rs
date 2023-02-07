//! CLI for the prime factorization program.
//!
//! Natural number N, given in range \[2, 2^128 - 1\], is decomposed
//! into a product of its prime factors, p_1^k_1 * p_2^k_2 * ... * p_m^k_m,
//! in which each p_ term represents a prime factor and the count of
//! its occurrence is marked by the corresponding k_ term.
//!
//! Following example shows how to call the program via command line,
//! assuming that the binary has been built first and made accessible
//! (i.e., is in $PATH)
//!
//! ```bash
//! prime_factorization num [-p | --pretty]
//! ```
//!
//! Number argument `num` is mandatory and must be in the determined range.
//!
//! If passing "pretty" print option, either `--pretty` or `-p` in arguments,
//! the output would be slightly different from the standard, namely giving
//! the prime factor representation in the same format as above:
//!
//! p_1^k_1 * p_2^k_2 * ... * p_m^k_m,
//!
//! and not just listing the factors in manner p_1, p_1, ..., p_m as they
//! are in the standard case.
//!
use std::{env, process};

extern crate prime_factorization;
use prime_factorization::{Factorization, UInt};

mod parser;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let (num, print_pretty): (u128, bool) =
        parser::parse_arguments(&mut args[1..]).unwrap_or_else(|err| {
            if err == "help" {
                process::exit(0);
            }
            eprintln!("Error with command line args: {err}");
            process::exit(1);
        });

    if num <= u32::MAX as u128 {
        factorize(num as u32, print_pretty);
    } else if num <= u64::MAX as u128 {
        factorize(num as u64, print_pretty);
    } else {
        factorize(num, print_pretty);
    }
}

fn factorize<T: 'static + UInt>(num: T, print_pretty: bool) {
    let factor_repr = Factorization::run(num);

    if factor_repr.factors.is_empty() {
        println!("no factors!");
        return;
    }

    if print_pretty {
        let repr = factor_repr.prime_factor_repr();

        let k = repr.len() - 1;

        let print_str_prefix = repr.iter().take(k).fold(String::new(), |acc, &(p, k)| {
            if k > 1 {
                format!("{}{}^{} * ", acc, &p.to_string(), &k.to_string())
            } else {
                format!("{}{} * ", acc, &p.to_string())
            }
        });

        let print_str_full = if repr[k].1 > 1 {
            format!("{}{}^{}", print_str_prefix, repr[k].0, repr[k].1)
        } else {
            format!("{}{}", print_str_prefix, repr[k].0)
        };

        println!("factors: {print_str_full}");
    } else {
        // Just print all factors on one line
        println!("{factor_repr}");
    }
}
