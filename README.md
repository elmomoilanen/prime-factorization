# Prime factorization

[![main](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml)
[![crate](https://img.shields.io/crates/v/prime_factorization.svg?logo=rust&color=orange)](https://crates.io/crates/prime_factorization)

This is a program to decompose a natural number N, up to `u128::MAX`, into a product of its prime factors. Based on the fundamental theorem of arithmetic, every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these prime numbers.

The factorization algorithm consists of trial division with the first one thousand primes, Fermat's factorization method, and Lenstra elliptic-curve factorization using projective coordinates with Suyama's parametrization. After Fermat's method and before advancing to the elliptic-curve factorization step, the possible primality of the number is checked using either the Miller-Rabin or strong Baillie-PSW primality test, depending on the magnitude of the number. The latter test is not deterministic in the number range it's used here (up to 128 bits) but there are no known counterexamples.

## Install ##

To install as a dependency (library target) for another program, add the following to your `Cargo.toml`

```toml
[dependencies]
prime_factorization = "1.0.5"
```

For the binary target, run the command `cargo install prime_factorization` and ensure that the installation location is in your PATH (i.e., Rust toolchain properly configured).

## Use ##

Use the library as follows

```rust
use prime_factorization::Factorization;

// Factorize the following semiprime
let num: u128 = 3_746_238_285_234_848_709_827;

let factor_repr = Factorization::run(num);

// Check that the returned factors are correct
assert_eq!(factor_repr.factors, vec![103_979, 36_028_797_018_963_913]);
```

Note that all integers from 2 to 2^128 - 1 can be factorized, but the used integer type must implement (alongside a few others) the trait *From\<u32\>*.

Sometimes it might be enough to check whether a particular number is a prime

```rust
use prime_factorization::Factorization;

let num: u128 = 332_306_998_946_228_968_225_951_765_070_086_139;

// Use the `is_prime` convenience field
assert_eq!(Factorization::run(num).is_prime, true);
```

If the binary target was installed, the CLI can be used as follows

```bash
prime_factorization num [-p | --pretty]
```

where the argument `num` is the mandatory natural number and the option *-p* or *--pretty* is a print flag which, when given, causes the output to be in the proper factor representation format $$p_1^{k_1} * ... * p_m^{k_m}$$. Without the flag, the output only lists all the prime factors from smallest to largest.

## Remarks ##

- Elliptic-curve factorization must use multithreading to be efficient. The thread count should be set to a value of at least two and preferably below the number of CPU cores to optimize performance. In terms of performance, a lower value (2-5) seems to be the best, but large 128 bit semiprimes could be factorized faster with a larger thread count based on benchmarking. The thread count can be changed by the *MAX_THREADS_* constants in the *factor* module.

- Miller-Rabin and Baillie-PSW primality tests are probabilistic but do not contain counterexamples in the number range this program uses. Elliptic-curve factorization uses random initial points on the curves, which can cause some deviation in execution times.

## License ##

This program is licensed under the [CC0v1](https://github.com/elmomoilanen/prime-factorization/blob/main/LICENSE).
