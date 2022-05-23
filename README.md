# Prime factorization

[![main](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml)

Command line program to decompose a natural number N into a product of its prime factors. Based on the fundamental theorem of arithmetic every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these prime numbers.

Factorization algorithm of this program consists of trial division with the first one-thousand primes, Fermat's factorization and Lenstra elliptic-curve factorization. The elliptic-curve method uses projective coordinates with Suyama's parametrization and as a default 8 worker threads. This worker count can be changed by modifying the *MAX_WORKERS* constant in the code file *factorization.rs*.

Primality verification after trial division and before elliptic-curve factorization is conducted either with Miller-Rabin or strong Baillie-PSW primality test depending on the magnitude of the number. Latter test is not deterministic but there aren't known counterexamples in the allowed range for the input in this program.

For more information of factorization and primality testing, see the module documentations.

## Use ##

This program takes one integer as an input that must be in the interval [2, 2^128 - 1]. Factorization of this integer should be completed relatively fast (of course, depends also on the underlying machine), the most difficult integers to factorize being semiprimes (products of two primes of similar size).

Following command illustrates a direct way to run the program (with Cargo)

```bash
cargo run --release N
```

Another usage option in which the build step has been separated is the following

```bash
cargo build --release
./target/release/prime_factorization N
```

where the Cargo's build command is necessary only prior to the first run.

To give few concrete examples, consider first e.g. integer 1729 for which this program

```bash
./target/release/prime_factorization 1729
```

would return the prime factors 7, 13 and 19 printed on single line after the command. It's also possible to get the output printed as factor representation $7^1 + 13^1 + 19^1$ by passing an option *--pretty* or shortly *-p* before or after the integer.

For another example, if calling the program with the largest 63 bit number

```bash
./target/release/prime_factorization $((2 ** 63 - 1))
```

returned output would contain the factors 7, 7, 73, 127, 337, 92737 and 649657. Notice from this example how the output contains all terms to restore the original input number (as a product of these factors), not just the unique factors.

Unit/functional tests (implemented inside modules) can be run by command

```bash
cargo test
```

and they should finish pretty fast if the invoker has a decent amount of computing power.
