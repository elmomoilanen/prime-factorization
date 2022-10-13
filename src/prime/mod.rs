//! Primality testing for odd natural (unsigned) numbers.
//!
//! Prior calling the public entrypoint `is_odd_prime_factor` of this module,
//! the number should have been cheched with the trial division primality test
//! with a few of the smallest prime numbers. Calling the primality test directly
//! here would fail e.g. for number seven because it's one of the base elements
//! of the Miller-Rabin primality test. According to unit tests contained in this
//! module, the primality test works correctly from number (prime) 67 onwards.
//!
//! Primality testing is separated in the following manner:
//! - Numbers having 64 or less bits are cheched with the Miller-Rabin test.
//! - Larger numbers up to 128 bits are cheched with the strong Baillie-PSW test.
//!
//! Baillie-PSW primality test is not deterministic but there are not known counterexamples in the range
//! this program uses (numbers up to 128 bits).
//!
use std::cmp::Ordering;
use std::convert::Into;

use num::{integer, PrimInt};

use crate::{
    arith::{Arith, CoreArith},
    UInt,
};

struct LucasParams<T: UInt>(T, T, T);

pub fn is_odd_prime_factor<T: UInt>(num: T) -> bool {
    if num <= T::one() || num & T::one() == T::zero() {
        return false;
    }
    // Assuming num >= 67 so MR-test works correctly

    let num_u128: u128 = num.into();

    if num_u128 > u64::MAX as u128 {
        is_prime_strong_bpsw(num_u128)
    } else {
        match num_u128.cmp(&(u32::MAX as u128)) {
            Ordering::Less | Ordering::Equal => {
                let mr_base_small: [u32; 3] = [2, 7, 61];
                is_prime_mr(num, &mr_base_small[..])
            }
            _ => {
                let mr_base_large: [u32; 7] =
                    [2, 325, 9375, 28_178, 450_775, 9_780_504, 1_795_265_022];
                is_prime_mr(num, &mr_base_large[..])
            }
        }
    }
}

fn is_prime_mr<T: UInt>(num: T, bases: &[u32]) -> bool {
    let num_even = num - T::one();

    let pow = num_even.trailing_zeros();
    let num_odd = num_even.unsigned_shr(pow);
    // num_even = 2^pow * num_odd

    'base: for base in bases.iter() {
        let mut q = T::exp_mod((*base).into(), num_odd, num);

        if q == T::one() || q == num_even {
            continue;
        }

        for _ in 1..pow {
            q = T::mult_mod_unsafe(q, q, num);

            if q == num_even {
                continue 'base;
            }
        }

        return false;
    }

    true
}

fn is_prime_strong_bpsw(num: u128) -> bool {
    let mr_test_base: [u32; 1] = [2];

    if !is_prime_mr(num, &mr_test_base[..]) {
        return false;
    }

    if num == i128::MAX as u128 {
        return true;
    }

    match select_lucas_params(num) {
        Some(params) => pass_strong_lucas_test(num, params),
        None => false,
    }
}

fn select_lucas_params(num: u128) -> Option<LucasParams<u128>> {
    let d_seq = (5..).step_by(2).enumerate();

    for (i, mut d) in d_seq {
        // num > u64::MAX, thus case d > num is highly unlikely (should never occur)
        let d_orig = d;

        if i & 1 == 1 {
            d = num - d;
        }

        let jac_sym = u128::jacobi_symbol(d, num);

        if jac_sym == -1 {
            let (p, q) = if i & 1 == 1 {
                (1, (1 + d_orig) >> 2)
            } else if d == 5 {
                (5, 5)
            } else {
                (1, num - ((d_orig - 1) >> 2))
            };
            return Some(LucasParams(d, p, q));
        }

        if jac_sym == 0 && d_orig != num {
            return None;
        }

        if i == 10 {
            let num_sqrt = integer::sqrt(num);
            if num_sqrt * num_sqrt == num {
                return None;
            }
        }
    }

    None
}

fn pass_strong_lucas_test(num: u128, params: LucasParams<u128>) -> bool {
    let num_even = num + 1; // Not allowed with u128::MAX but Fermat's test should have handled it
    let num_odd = num_even.unsigned_shr(num_even.trailing_zeros());
    let num_even_lead_zeros = num_even.leading_zeros();

    let bits_to_check = u128::BITS - num_even_lead_zeros;
    let num_even_rev = num_even.reverse_bits() >> num_even_lead_zeros;

    let LucasParams(_, _, luc_q) = params;
    let (mut luc_u, mut luc_v, mut luc_w) = (0, 2, 1);

    let (mut round, euler_check_round) = (0, num_even >> 1);
    let (mut is_slprp, mut pass_euler_crit) = (false, false);

    for bit in 0..bits_to_check {
        if bit > 0 {
            update_lucas_normal_uvq(num, &mut luc_u, &mut luc_v, &mut luc_w);
            round *= 2;
        }

        if !is_slprp && luc_v == 0 && round > num_odd && bit < bits_to_check - 1 {
            is_slprp = true;
        }

        if (num_even_rev >> bit) & 1 == 1 {
            update_lucas_odd_bit_uvq(num, &params, &mut luc_u, &mut luc_v, &mut luc_w);
            round += 1;
        }

        if round == num_odd && (luc_u == 0 || luc_v == 0) {
            is_slprp = true;
        }

        if round == euler_check_round {
            let luc_q_jac: u128 = match u128::jacobi_symbol(luc_q, num).cmp(&0) {
                Ordering::Equal => 0,
                Ordering::Greater => num - luc_q,
                Ordering::Less => luc_q,
            };

            if u128::add_mod_unsafe(luc_w, luc_q_jac, num) == 0 {
                pass_euler_crit = true;
            }
        }
    }

    if luc_u != 0 || !is_slprp || !pass_euler_crit {
        return false;
    }

    if u128::mult_mod_unsafe(2, luc_q, num) != luc_v {
        return false;
    }

    true
}

fn update_lucas_normal_uvq(num: u128, u: &mut u128, v: &mut u128, w: &mut u128) {
    *u = u128::mult_mod_unsafe(*u, *v, num);

    *v = u128::add_mod_unsafe(
        u128::mult_mod_unsafe(*v, *v, num),
        u128::mult_mod_unsafe(num - 2, *w, num),
        num,
    );

    *w = u128::mult_mod_unsafe(*w, *w, num);
}

fn modify_lucas_coef(x_left: u128, x_right: u128, num: u128) -> u128 {
    let numer = u128::add_mod_unsafe(x_left, x_right, num);

    if numer & 1 == 1 {
        // Decompose both odds to 2k + 1, and compute k1 + k2 + 1 (mod num)
        u128::add_mod_unsafe((numer - 1) >> 1, ((num - 1) >> 1) + 1, num)
    } else {
        numer >> 1
    }
}

fn update_lucas_odd_bit_uvq(
    num: u128,
    params: &LucasParams<u128>,
    u: &mut u128,
    v: &mut u128,
    w: &mut u128,
) {
    let LucasParams(d, p, q) = *params;

    let new_u = modify_lucas_coef(u128::mult_mod_unsafe(p, *u, num), *v, num);

    let new_v = modify_lucas_coef(
        u128::mult_mod_unsafe(d, *u, num),
        u128::mult_mod_unsafe(p, *v, num),
        num,
    );

    *u = new_u;
    *v = new_v;
    *w = u128::mult_mod_unsafe(q, *w, num);
}

#[cfg(test)]
mod tests;
