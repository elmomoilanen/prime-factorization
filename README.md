# Prime factorization

[![main](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml/badge.svg)](https://github.com/elmomoilanen/prime-factorization/actions/workflows/main.yml)

Program to decompose a natural number N into a product of its prime factors. Based on the fundamental theorem of arithmetic every natural number larger than one is either a prime itself or can be represented as a product of primes that is unique up to the order of these prime numbers.

Factorization algorithm of this program consists of trial division with the first one-thousand primes, Fermat's factorization and Lenstra elliptic-curve factorization. The elliptic-curve method uses projective coordinates with Suyama's parametrization and by default 6 threads. This can be changed by the *MAX_WORKERS* constant in the `factor` module but its value must be two at least (otherwise performance will deteriorate notably).

Primality verification after trial division and before elliptic-curve factorization is conducted either with Miller-Rabin or strong Baillie-PSW primality test depending on the magnitude of the number. Latter test is not deterministic in the number range it's used here but there aren't known counterexamples.

## Install ##

To be added.

## Use ##

This program takes one integer N as an input that must be in the interval [2, 2^128 - 1]. Factorization of this integer should be completed relatively fast, the most difficult integers to factorize being semiprimes (products of two primes of similar size that are not too close to each others).

More to be added.
