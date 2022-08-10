# Prime factorization

[![main](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml)
[![crate](https://img.shields.io/crates/v/prime_factorization.svg?logo=rust&color=orange)](https://crates.io/crates/prime_factorization)

Program to decompose a natural number N, up to `u128::MAX`, into a product of its prime factors. Based on the fundamental theorem of arithmetic every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these prime numbers.

Factorization algorithm of this program consists of trial division with the first one-thousand primes, Fermat's factorization and Lenstra elliptic-curve factorization using projective coordinates with Suyama's parametrization. After Fermat's and before elliptic-curve factorization step, possible primality of the number is checked and this is conducted either with Miller-Rabin or strong Baillie-PSW primality test depending on the magnitude of the number. Latter test is not deterministic in the number range it's used here (up to 128 bits) but there aren't known counterexamples.

## Install ##

To install as a dependency (library target) for some other program, add the following to your `Cargo.toml`

```toml
[dependencies]
prime_factorization = "^1.0"
```

For the binary target, run command `cargo install prime_factorization` and make sure that the installation location is in $PATH (Rust toolchain properly configured).

## Use ##

Use the library as follows

```rust
use prime_factorization::Factorization;

// Factorize following semiprime
let num: u128 = 3_746_238_285_234_848_709_827;

let factor_repr = Factorization::run(num);

// Check that the returned factors are correct
assert_eq!(factor_repr.factors, vec![103_979, 36_028_797_018_963_913]);
```

Notice that numbers from 2 to 2^128 - 1 can be factorized but the used integer type must implement (alongside few others) trait *From\<u32\>*.

Sometimes it might be enough to check whether a particular number is a prime

```rust
use prime_factorization::Factorization;

let num: u128 = 332_306_998_946_228_968_225_951_765_070_086_139;

// Use the `is_prime` convenience field
assert_eq!(Factorization::run(num).is_prime, true);
```

CLI can be used as follows

```bash
prime_factorization num [-p | --pretty]
```

where argument `num` is the mandatory natural number and option *-p* or *--pretty* is a print flag which, when given, causes the output to be in the proper factor representation format $$p_1^{k_1} * ... * p_m^{k_m}$$ Without the flag, output only lists all the prime factors from the smallest to largest.

## Dev remarks ##

- Elliptic-curve factorization must use few worker threads to be efficient. Default thread count is five which happened to be the most effective by rough empirical testing during development period. Thread count can be changed by the *MAX_WORKERS* constant in the *factor* module but its value must be two at least (otherwise performance will deteriorate notably).

- Miller-Rabin and Baillie-PSW primality tests are probabilistic but do not contain counterexamples in the number range this program uses. Elliptic-curve factorization uses random initial points on the curves that causes slight deviations to execution times.

## License ##

This program is licensed under the [CC0v1](https://github.com/elmomoilanen/Modular-equations/blob/main/LICENSE).
