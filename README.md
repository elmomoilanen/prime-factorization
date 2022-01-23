# Prime factorization

Command line program to decompose a natural number N into a product of its prime factors. Based on the fundamental theorem of arithmetic every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these primes.

Factorization algorithm consists of trial division, Fermat's factorization and Lenstra elliptic-curve factorization. Elliptic-curve method uses projective coordinates with Suyama's parametrization and by default 8 worker threads (rough idea is to have two threads per core). This worker count can be changed by tweaking the *MAX_WORKERS* constant.

Primality verification after trial division is done either with Miller-Rabin or strong Baillie-PSW test depending on the magnitude of the number.

For more information of factorization and primality testing, see the module documentations.

## Use ##

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

To give a concrete example, consider e.g. the number 1729 for which this program

```bash
./target/release/prime_factorization 1729
```

would return the factors 7, 13 and 19.

Unit tests (inside modules) can be run by command

```bash
cargo test
```

and they should finish within one minute.
