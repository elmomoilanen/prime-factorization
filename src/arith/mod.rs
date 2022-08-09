//! Basic modular arithmetic operations.
//!
//! It's recommended to use functions of `Arith` trait unless it is
//! guaranteed that the operands are less than the modulus `modu`.
//! Violating this constraint might cause two's complement wrapping.
//!

use std::cmp::{self, Ordering};
use std::mem;

use num::{PrimInt, Unsigned};

pub trait CoreArith<T: PrimInt + Unsigned> {
    fn add_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x < modu - y {
            x + y
        } else {
            cmp::min(x, y) - (modu - cmp::max(x, y))
        }
    }

    fn sub_mod_unsafe(x: T, y: T, modu: T) -> T {
        if x >= y {
            x - y
        } else {
            modu - (y - x)
        }
    }

    fn mult_mod_unsafe(mut x: T, mut y: T, modu: T) -> T {
        if x == T::zero() {
            return x;
        }

        let mut res = T::zero();

        while y > T::zero() {
            if y & T::one() == T::one() {
                res = Self::add_mod_unsafe(res, x, modu);
            }

            y = y.unsigned_shr(1);
            x = Self::add_mod_unsafe(x, x, modu);
        }

        res
    }

    fn exp_mod_unsafe(mut base: T, mut ex: T, modu: T) -> T {
        if base == T::zero() {
            return base;
        }

        let mut res = T::one();

        while ex > T::zero() {
            if ex & T::one() == T::one() {
                res = Self::mult_mod_unsafe(res, base, modu);
            }

            ex = ex.unsigned_shr(1);
            base = Self::mult_mod_unsafe(base, base, modu);
        }

        res
    }
}

pub trait Arith<T>: CoreArith<T>
where
    T: PrimInt + Unsigned + From<u32>,
{
    fn add_mod(x: T, y: T, modu: T) -> T {
        match (x < modu, y < modu) {
            (true, true) => Self::add_mod_unsafe(x, y, modu),
            (true, false) => Self::add_mod_unsafe(x, y % modu, modu),
            (false, true) => Self::add_mod_unsafe(x % modu, y, modu),
            _ => Self::add_mod_unsafe(x % modu, y % modu, modu),
        }
    }

    fn sub_mod(x: T, y: T, modu: T) -> T {
        match (x < modu, y < modu) {
            (true, true) => Self::sub_mod_unsafe(x, y, modu),
            (true, false) => Self::sub_mod_unsafe(x, y % modu, modu),
            (false, true) => Self::sub_mod_unsafe(x % modu, y, modu),
            _ => Self::sub_mod_unsafe(x % modu, y % modu, modu),
        }
    }

    fn mult_mod(x: T, y: T, modu: T) -> T {
        match (x < modu, y < modu) {
            (true, true) => Self::mult_mod_unsafe(x, y, modu),
            (true, false) => Self::mult_mod_unsafe(x, y % modu, modu),
            (false, true) => Self::mult_mod_unsafe(x % modu, y, modu),
            _ => Self::mult_mod_unsafe(x % modu, y % modu, modu),
        }
    }

    fn exp_mod(base: T, ex: T, modu: T) -> T {
        if base < modu {
            Self::exp_mod_unsafe(base, ex, modu)
        } else {
            Self::exp_mod_unsafe(base % modu, ex, modu)
        }
    }

    fn gcd_mod(mut x: T, mut y: T) -> T {
        if x == T::zero() || y == T::zero() {
            return x | y;
        }

        let shift = (x | y).trailing_zeros();
        x = x.unsigned_shr(x.trailing_zeros());

        loop {
            y = y.unsigned_shr(y.trailing_zeros());
            if x > y {
                mem::swap(&mut x, &mut y);
            }
            y = y - x;
            if y == T::zero() {
                break x.unsigned_shl(shift);
            }
        }
    }

    fn multip_inv(mut x: T, modu: T) -> T {
        if x >= modu {
            x = x % modu;
        }

        let (mut rem, mut rem_new) = (modu, x);
        let (mut inv, mut inv_new) = (T::zero(), T::one());

        while rem_new > T::zero() {
            let quo = rem / rem_new;

            let rem_temp = rem_new;
            rem_new = rem - quo * rem_new;
            rem = rem_temp;

            let inv_temp = inv_new;
            inv_new = Self::sub_mod_unsafe(inv, Self::mult_mod_unsafe(quo, inv_new, modu), modu);
            inv = inv_temp;
        }

        if rem > T::one() {
            // inverse doesn't exist, gcd(x, modu) > 1
            return T::zero();
        }

        inv
    }

    fn jacobi_symbol(mut x: T, mut n: T) -> i8 {
        if x >= n {
            x = x % n;
        }

        let mut par_t = 1;

        while x > T::zero() {
            while x & T::one() == T::zero() {
                x = x.signed_shr(1);

                let par_r = n & 7.into();
                if par_r == 3.into() || par_r == 5.into() {
                    par_t = -par_t;
                }
            }

            mem::swap(&mut x, &mut n);

            if (x & 3.into()) == 3.into() && (n & 3.into()) == 3.into() {
                par_t = -par_t;
            }
            x = x % n;
        }

        if n == T::one() {
            par_t
        } else {
            0
        }
    }

    fn trunc_square(x: T) -> T {
        match x.cmp(&T::zero()) {
            Ordering::Greater => {
                if x < T::max_value() / x {
                    x * x
                } else {
                    T::zero()
                }
            }
            _ => T::zero(),
        }
    }
}

#[cfg(test)]
mod tests;
