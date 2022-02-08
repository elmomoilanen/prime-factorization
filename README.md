# Prime factorization

[![main](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml)

Command line program to decompose a natural number N into a product of its prime factors. Based on the fundamental theorem of arithmetic every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these primes.

The factorization algorithm of this program consists of trial division with the first one-thousand primes, Fermat's factorization and Lenstra elliptic-curve factorization. The elliptic-curve method uses projective coordinates with Suyama's parametrization and as a default 8 worker threads (rough idea is to have two threads per core). This worker count can be changed by tweaking the *MAX_WORKERS* constant in code file *factorization.rs*.

Primality verification after trial division and before elliptic-curve factorization in particular is done either with Miller-Rabin or strong Baillie-PSW test depending on the magnitude of the number. Latter is not deterministic but there aren't known counterexamples in the allowed range for input.

For more information of factorization and primality testing, see the module documentations.

## Use ##

This program takes one integer input that must be in the interval [2, 2^128 - 1]. Factorization of this integer should be completed relatively fast (of course, depends also on the underlying machine), most difficult integers to factorize being semiprimes (products of two primes of similar size).

Following example illustrates a direct way to run the program (with Cargo)

```bash
cargo run --release N
```

Another usage option is the following

```bash
cargo build --release
./target/release/prime_factorization N
```

where the Cargo's build step is necessary only prior to the first run.

To give few concrete examples, consider first e.g. the number 1729 for which this program

```bash
./target/release/prime_factorization 1729
```

would return the factors 7, 13 and 19.

For another example, if calling the program with the largest 63 bit number

```bash
./target/release/prime_factorization $((2 ** 63 - 1))
```

returned output would be the factors 7, 7, 73, 127, 337, 92737 and 649657. Notice from this example that the returned output contains all terms to restore the original input number (as a product of these factors), not just unique factors.

Unit/functional tests (inside modules) can be run by command

```bash
cargo test
```

and they should finish within couple of minutes.
