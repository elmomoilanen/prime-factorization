//! Library for prime factorization up to 128 bit integers.
//!
//! Natural number N, given in range \[2, 2^128 - 1\], is decomposed into a product
//! of its prime factors, p_1^k_1 * p_2^k_2 * ... * p_m^k_m, in which each p_ term
//! represents a prime factor and the count of its occurrence is marked by the
//! corresponding k_ term.
//!
//! This decomposition is of huge importance as based on the fundamental theorem
//! of arithmetic every natural number larger than one is either a prime itself
//! or can be represented as a product of primes that is unique up to the order
//! of these prime numbers.
//!
//! E.g., a natural number 30 has the prime factor representation 2 * 3 * 5
//!
//! ```
//! use prime_factorization::Factorization;
//!
//! // `factor_repr` is now an instance of the `Factorization` struct
//! let factor_repr = Factorization::run(30u32);
//!
//! // Check that the factors are correct
//! assert_eq!(factor_repr.factors, vec![2, 3, 5]);
//!
//! // Given these factors, the number is certainly not a prime
//! assert_eq!(factor_repr.is_prime, false);
//! ```
//!
//! For natural number 3773, which prime factor representation is 7 * 7 * 7 * 11,
//! or 7^3 * 11 more densely written, the result would be given as follows
//!
//! ```
//! use prime_factorization::Factorization;
//!
//! let factor_repr = Factorization::<u32>::run(3773);
//!
//! assert_eq!(factor_repr.factors, vec![7, 7, 7, 11]);
//! ```
//!
//! which makes it clear that the returned output (prime factors) contains all terms
//! to restore the original input number (as a product of these factors), not just
//! the unique factors 7 and 11.
//!
//! If the input number is a prime, like in the following example, the returned factors
//! would only contain this prime number and the field `is_prime` would be true.
//!
//! ```
//! use prime_factorization::Factorization;
//!
//! let prime: u128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
//! let factor_repr = Factorization::run(prime);
//!
//! assert_eq!(factor_repr.factors, vec![prime]);
//!
//! assert_eq!(factor_repr.is_prime, true);
//! ```
//!
//! The largest number that can be factorized with this program, has the following
//! prime factor representation
//!
//! ```
//! use prime_factorization::Factorization;
//!
//! let factor_repr = Factorization::run(u128::MAX);
//!
//! let correct_factors = vec![
//!     3, 5, 17, 257, 641, 65_537, 274_177, 6_700_417, 67_280_421_310_721
//! ];
//!
//! assert_eq!(factor_repr.factors, correct_factors);
//! ```
//!
use std::fmt::{Debug, Display};
use std::marker::{Send, Sync};

use num::{integer::Roots, PrimInt, Unsigned};

mod arith;
mod elliptic;
mod factor;
mod prime;

pub trait UInt:
    PrimInt + Unsigned + Display + Debug + Roots + Send + Sync + From<u32> + Into<u128>
{
}

impl<T> UInt for T where
    T: PrimInt + Unsigned + Display + Debug + Roots + Send + Sync + From<u32> + Into<u128>
{
}

impl<T> arith::CoreArith<T> for T where T: UInt {}
impl<T> arith::Arith<T> for T where T: UInt {}

pub use factor::Factorization;
