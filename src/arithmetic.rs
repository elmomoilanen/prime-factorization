//! Modular arithmetic functionality.
//!
//! - Implements ordinary arithmetic operations addition, multiplication etc.
//! - Other: Jacobi symbol, greatest common divisor (gcd) and multiplicative inverse
//!
use std::cmp::Ordering;
use std::{cmp, mem};

use num::{PrimInt, Unsigned};

pub fn mod_add<T>(x: T, y: T, modu: T) -> T
where
    T: PrimInt + Unsigned,
{
    let x_mod = x % modu;
    let y_mod = y % modu;

    if x_mod < modu - y_mod {
        x_mod + y_mod
    } else {
        cmp::min(x_mod, y_mod) - (modu - cmp::max(x_mod, y_mod))
    }
}

pub fn mod_sub<T>(x: T, y: T, modu: T) -> T
where
    T: PrimInt + Unsigned,
{
    let x_mod = x % modu;
    let y_mod = y % modu;

    if x_mod >= y_mod {
        x_mod - y_mod
    } else {
        modu - (y_mod - x_mod)
    }
}

pub fn mod_mult<T>(x: T, y: T, modu: T) -> T
where
    T: PrimInt + Unsigned,
{
    let (zero, one) = (T::zero(), T::one());

    let mut x_mod = x % modu;
    let mut y_mod = y % modu;

    if y_mod == zero || x_mod == zero {
        return zero;
    }

    let mut res = zero;

    while y_mod > zero {
        if y_mod & one != zero {
            res = mod_add(res, x_mod, modu);
        }
        y_mod = y_mod.unsigned_shr(1);
        x_mod = mod_add(x_mod, x_mod, modu);
    }

    res
}

pub fn mod_exp<T>(mut base: T, mut exp: T, modu: T) -> T
where
    T: PrimInt + Unsigned,
{
    let (zero, one) = (T::zero(), T::one());

    base = base % modu;
    if base == zero {
        return base;
    }

    let mut res = one;

    while exp > zero {
        if exp & one != zero {
            res = mod_mult(res, base, modu);
        }
        exp = exp.unsigned_shr(1);
        base = mod_mult(base, base, modu);
    }

    res
}

pub fn trunc_square<T>(x: T) -> T
where
    T: PrimInt + Unsigned,
{
    let zero = T::zero();

    match x.cmp(&zero) {
        Ordering::Equal => zero,
        _ => {
            if x < T::max_value() / x {
                x * x
            } else {
                zero
            }
        }
    }
}

pub fn jacobi_symbol<T>(mut x: T, mut n: T) -> i8
where
    T: PrimInt + Unsigned + From<u32>,
{
    if x >= n {
        x = x % n;
    }

    let (zero, one) = (T::zero(), T::one());
    let (three, five, seven) = (3.into(), 5.into(), 7.into());

    let mut param_t = 1;

    while x > zero {
        while x & one == zero {
            x = x.signed_shr(1);

            let param_r = n & seven;
            if param_r == three || param_r == five {
                param_t = -param_t;
            }
        }

        mem::swap(&mut x, &mut n);

        if (x & three) == three && (n & three) == three {
            param_t = -param_t;
        }
        x = x % n;
    }

    if n == one {
        param_t
    } else {
        0
    }
}

pub fn gcd<T>(mut x: T, mut y: T) -> T
where
    T: PrimInt + Unsigned,
{
    let zero = T::zero();

    if x == zero || y == zero {
        return x | y;
    }

    let shift = (x | y).trailing_zeros();
    x = x.unsigned_shr(x.trailing_zeros());

    loop {
        y = y.unsigned_shr(y.trailing_zeros());
        if x > y {
            mem::swap(&mut y, &mut x);
        }
        y = y - x;
        if y == zero {
            break x.unsigned_shl(shift);
        }
    }
}

pub fn multip_inv<T>(mut x: T, modu: T) -> T
where
    T: PrimInt + Unsigned,
{
    let (zero, one) = (T::zero(), T::one());

    if x >= modu {
        x = x % modu;
    }

    let (mut r_prev, mut r_curr) = (modu, x);
    let (mut t_prev, mut t_curr) = (zero, one);

    while r_curr > zero {
        let quo = r_prev / r_curr;

        let r_temp = r_curr;
        r_curr = r_prev - quo * r_curr;
        r_prev = r_temp;

        let t_temp = t_curr;
        t_curr = mod_sub(t_prev, mod_mult(quo, t_curr, modu), modu);
        t_prev = t_temp;
    }

    if r_prev > one {
        // inverse doesn't exist, gcd(x, modu) > 1
        return zero;
    }

    t_prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_add_small_modu() {
        let modu = 5;

        let test_cases: [[u32; 3]; 10] = [
            // [x, y, res]: x + y = res (mod modu)
            [0, 0, 0],
            [1, 2, 3],
            [2, 1, 3],
            [2, 2, 4],
            [3, 2, 0],
            [2, 3, 0],
            [6, 1, 2],
            [1, 6, 2],
            [11, 7, 3],
            [7, 11, 3],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_add(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_add_large_modu() {
        let modu = u64::MAX;

        let test_cases: [[u64; 3]; 5] = [
            // [x, y, res]: x + y = res (mod modu)
            [modu - 1, 2, 1],
            [modu - 5, 5, 0],
            [modu - 2, modu - 2, modu - 4],
            [modu - 1, modu - 1, modu - 2],
            [modu, modu - 1, modu - 1],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_add(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_sub_small_modu() {
        let modu = 7;

        let test_cases: [[u32; 3]; 10] = [
            // [x, y, res]: x - y = res (mod modu)
            [0, 0, 0],
            [11, 9, 2],
            [5, 2, 3],
            [2, 5, 4],
            [6, 7, 6],
            [1, 7, 1],
            [7, 1, 6],
            [0, 6, 1],
            [15, 1, 0],
            [1, 15, 0],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_sub(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_mult_small_modu() {
        let modu = 5;

        let test_cases: [[u32; 3]; 10] = [
            // [x, y, res]: x * y = res (mod modu)
            [0, 0, 0],
            [0, 1, 0],
            [1, 1, 1],
            [2, 2, 4],
            [3, 2, 1],
            [2, 3, 1],
            [11, 7, 2],
            [7, 11, 2],
            [7, 8, 1],
            [8, 7, 1],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_mult(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_mult_large_modu() {
        let modu = u128::MAX;

        let test_cases: [[u128; 3]; 3] = [
            // [x, y, res]: x * y = res (mod modu)
            [modu - 1, modu - 1, 1],
            [modu - 1, 1, modu - 1],
            [modu - 2, modu - 1, 2],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_mult(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_exp_small_modu() {
        let modu = 5;

        let test_cases: [[u32; 3]; 10] = [
            // [x, y, res]: x^y = res (mod modu)
            [0, 0, 0],
            [0, 1, 0],
            [1, 0, 1],
            [5, 1, 0],
            [2, 4, 1],
            [4, 2, 1],
            [3, 4, 1],
            [4, 3, 4],
            [4, 40, 1],
            [8, 50, 4],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_exp(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_exp_large_modu() {
        let modu = u64::MAX;

        let test_cases: [[u64; 3]; 3] = [
            // [x, y, res]: x^y = res (mod modu)
            [2, 1_000_000_000, 1],
            [modu - 1, 1_000_000_000, 1],
            [modu - 1, 1_000_000_001, modu - 1],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_exp(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn mod_exp_large_modu_other() {
        let modu = i64::MAX as u64;

        let test_cases: [[u64; 3]; 3] = [
            // [x, y, res]: x^y = res (mod modu)
            [2, 9_999_999, 512],
            [9_987_654, 999_999_901_010_111, 2_940_910_929_841_963_431],
            [modu - 1, 100_000, 1],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);

            assert_eq!(mod_exp(x, y, modu), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn jacobi_symbol_small_operands() {
        let test_cases: [(u32, u32, i8); 15] = [
            (1, 1, 1),
            (15, 1, 1),
            (2, 3, -1),
            (29, 9, 1),
            (4, 11, 1),
            (17, 11, -1),
            (19, 29, -1),
            (10, 33, -1),
            (11, 33, 0),
            (12, 33, 0),
            (14, 33, -1),
            (15, 33, 0),
            (15, 37, -1),
            (29, 59, 1),
            (30, 59, -1),
        ];

        for case in test_cases.iter() {
            let (x, n, res) = case;

            assert_eq!(jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
        }
    }

    #[test]
    fn jacobi_symbol_large_operands() {
        let max_i128 = i128::MAX as u128;

        let test_cases: [(u128, u128, i8); 4] = [
            (1_241_942_351, 2_147_483_647, 1),
            (99, max_i128, 1),
            (max_i128 - 1, max_i128, -1),
            (max_i128, max_i128, 0),
        ];

        for case in test_cases.iter() {
            let (x, n, res) = case;

            assert_eq!(jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
        }
    }

    #[test]
    fn trunc_square_u64() {
        let test_cases: [[u64; 2]; 5] = [
            [0, 0],
            [1, 1],
            [2, 4],
            [u32::MAX as u64, 18_446_744_065_119_617_025],
            [u32::MAX as u64 + 1, 0],
        ];

        for case in test_cases.iter() {
            assert_eq!(trunc_square(case[0]), case[1], "x: {}", case[0]);
        }
    }

    #[test]
    fn gcd_mix() {
        let test_cases: [[u64; 3]; 15] = [
            [1, 0, 1],
            [0, 1, 1],
            [2, 3, 1],
            [3, 2, 1],
            [34, 85, 17],
            [224, 412, 4],
            [526, 17_210, 2],
            [10_500, 975, 75],
            [100_000, 15_888, 16],
            [900, 999_888_000, 300],
            [1_001_116_321, 10_011_18_301, 1],
            [9_223_372_036_854_775_807, 3, 1],
            [9_223_372_036_854_775_807, 9_933_434_335_423, 73],
            [18_446_744_073_709_551_615, 1_640_877_430_502_539, 17],
            [18_446_744_073_709_551_615, 572_590_724_124, 3],
        ];

        for case in test_cases.iter() {
            let (x, y) = (case[0], case[1]);
            assert_eq!(gcd(x, y), case[2], "x: {}, y: {}", x, y);
        }
    }

    #[test]
    fn gcd_equality() {
        let test_cases: [u128; 3] = [5, 16_358_049_139, u128::MAX];

        for case in test_cases.iter() {
            let x = *case;
            assert_eq!(gcd(x, x), x);
        }
    }

    #[test]
    fn multip_inverse_exist() {
        let test_cases: [[u64; 3]; 8] = [
            // [a, m, x] s.t. a*x = 1 (mod m) is satisfied
            [5, 11, 9],
            [8, 11, 7],
            [10, 11, 10],
            [3, 5000, 1667],
            [1667, 5000, 3],
            [999, 5000, 3999],
            [999, 9_223_372_036_854_775_807, 3_619_181_019_466_538_655],
            [
                9_223_372_036_854_775_804,
                9_223_372_036_854_775_807,
                3_074_457_345_618_258_602,
            ],
        ];

        for case in test_cases.iter() {
            let (a, modu) = (case[0], case[1]);
            assert_eq!(multip_inv(a, modu), case[2], "a: {}, mod: {}", a, modu);
        }
    }

    #[test]
    fn multip_inverse_not_exists() {
        let test_cases: [[u64; 2]; 4] = [[5, 5000], [50, 5000], [55, 5000], [0, 5000]];

        for case in test_cases.iter() {
            let (a, modu) = (case[0], case[1]);
            assert_eq!(multip_inv(a, modu), 0, "a: {}, mod: {}", a, modu);
        }
    }
}
