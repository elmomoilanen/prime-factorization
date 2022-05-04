//! Primality testing for odd natural (unsigned) numbers.
//!
//! Prior calling the public entrypoint `is_odd_prime_factor` of this module,
//! the number should have been cheched with the trial division primality test
//! with few of the smallest prime numbers, e.g. 7. Calling the primality test
//! here would fail for number seven because it's one of the base elements of the Miller-
//! Rabin primality test. According to unit tests contained in this module, the primality
//! test works correctly from number (prime) 67 onwards.
//!
//! Primality testing is separated in the following manner:
//! - Numbers having 64 or less bits are cheched with the Miller-Rabin test.
//! - Larger numbers up to 128 bits are cheched with the strong Baillie-PSW test.
//!
use std::cmp::Ordering;
use std::convert::Into;

use num::{integer, PrimInt};

use crate::{arith, factorization::UInt};

struct LucasParams<T: UInt>(T, T, T);

pub fn is_odd_prime_factor<T: UInt>(num: T) -> bool {
    if num & T::one() == T::zero() {
        return false;
    }

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
                let mr_base_large: [u32; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
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

    for base in bases.iter() {
        let mut q = arith::mod_exp((*base).into(), num_odd, num);

        if q == T::one() || q == num_even {
            continue;
        }

        let mut jump = false;

        for _ in 1..pow {
            q = arith::mod_mult(q, q, num);

            if q == num_even {
                jump = true;
                break;
            }
        }

        if jump {
            continue;
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
        let d_orig = d;

        if i & 1 == 1 {
            d = num - d % num;
        }

        let jac_sym = arith::jacobi_symbol(d, num);

        if jac_sym == -1 {
            let (p, q) = if i & 1 == 1 {
                (1, (1 + d_orig) >> 2)
            } else if d == 5 {
                (5, 5)
            } else {
                let q_temp = (d_orig - 1) >> 2;
                (1, num - q_temp % num)
            };
            return Some(LucasParams(d, p, q));
        }

        if jac_sym == 0 && (d_orig < num || d_orig % num != 0) {
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
    let num_even = num + 1; // cannot be done with u128::MAX
    let num_odd = num_even.unsigned_shr(num_even.trailing_zeros());
    // num_even = 2^pow * num_odd, for pow == num_even.trailing_zeros()

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
            let luc_q_jac: u128 = match arith::jacobi_symbol(luc_q, num).cmp(&0) {
                Ordering::Equal => 0,
                Ordering::Greater => num - luc_q % num,
                Ordering::Less => luc_q,
            };

            if arith::mod_add(luc_w, luc_q_jac, num) == 0 {
                pass_euler_crit = true;
            }
        }
    }

    if luc_u != 0 || !is_slprp || !pass_euler_crit {
        return false;
    }

    if arith::mod_mult(2, luc_q, num) != luc_v % num {
        return false;
    }

    true
}

fn update_lucas_normal_uvq(num: u128, u: &mut u128, v: &mut u128, w: &mut u128) {
    *u = arith::mod_mult(*u, *v, num);

    *v = arith::mod_add(
        arith::mod_mult(*v, *v, num),
        arith::mod_mult(num - 2, *w, num),
        num,
    );

    *w = arith::mod_mult(*w, *w, num);
}

fn modify_lucas_coef(x_left: u128, x_right: u128, num: u128) -> u128 {
    let numer = arith::mod_add(x_left, x_right, num);

    if numer & 1 == 1 {
        // decompose both odds to 2k + 1, and compute k1 + k2 + 1 (mod num)
        arith::mod_add((numer - 1) >> 1, ((num - 1) >> 1) + 1, num)
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

    let new_u = modify_lucas_coef(arith::mod_mult(p, *u, num), *v, num);

    let new_v = modify_lucas_coef(
        arith::mod_mult(d, *u, num),
        arith::mod_mult(p, *v, num),
        num,
    );

    *u = new_u;
    *v = new_v;
    *w = arith::mod_mult(q, *w, num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_factor_small_odd_primes() {
        let test_primes: [u32; 10] = [67, 71, 73, 79, 83, 89, 97, 101, 103, 107];

        for prime in test_primes.iter() {
            assert_eq!(is_odd_prime_factor(*prime), true, "{}", *prime);
        }
    }

    #[test]
    fn is_prime_factor_small_range() {
        let start_num: u32 = 67;
        let stop_num = 108; // range [67, 108)

        // there should be ten primes within this range, see the previous test

        let prime_count = (start_num..stop_num)
            .filter(|x| is_odd_prime_factor(*x))
            .count();

        assert_eq!(prime_count, 10);
    }

    #[test]
    fn is_prime_factor_smaller_primes() {
        let test_primes: [u64; 25] = [
            7927,
            7933,
            7937,
            7949,
            8009,
            8191,
            16_369,
            131_071,
            319_993,
            999_331,
            15_485_863,
            256_203_221,
            633_910_099,
            982_451_653,
            2_147_483_647,
            4_294_967_291,
            50_000_038_603,
            549_755_813_881,
            36_028_797_018_963_913,
            72_057_594_037_927_931,
            2_305_843_009_213_693_951,
            9_223_372_036_854_775_337,
            9_223_372_036_854_775_783,
            18_446_744_073_709_551_533,
            18_446_744_073_709_551_557,
        ];

        for prime in test_primes.iter() {
            assert_eq!(is_odd_prime_factor(*prime), true, "{}", *prime);
        }
    }

    #[test]
    fn is_prime_factor_smaller_composites() {
        let test_composites: [u64; 15] = [
            1_795_265_021,
            1_795_265_022,
            1_795_265_023,
            2_147_483_643,
            4_294_967_293,
            10_449_049_901,
            150_267_335_403,
            430_558_874_533,
            35_184_372_088_697,
            50_131_820_635_651,
            936_916_995_253_453,
            25_012_804_853_117_569,
            9_223_372_036_854_775_781,
            9_223_372_036_854_775_806,
            9_223_372_036_854_775_807,
        ];

        for comp in test_composites.iter() {
            assert_eq!(is_odd_prime_factor(*comp), false, "{}", *comp);
        }
    }

    #[test]
    fn is_prime_factor_larger_primes() {
        let test_primes: [u128; 20] = [
            36_893_488_147_419_103_183,
            36_893_488_147_419_102_739,
            73_786_976_294_838_206_459,
            37_778_931_862_957_161_709_471,
            37_778_931_862_957_161_709_361,
            37_778_931_862_957_161_709_289,
            37_778_931_862_957_161_709_279,
            618_970_019_642_690_137_449_562_111,
            618_970_019_642_690_137_449_562_091,
            618_970_019_642_690_137_449_562_081,
            19_807_040_628_566_084_398_385_987_581,
            19_807_040_628_566_084_398_385_987_573,
            2_535_301_200_456_458_802_993_406_410_683,
            2_535_301_200_456_458_802_993_406_410_653,
            2_535_301_200_456_458_802_993_406_410_539,
            2_535_301_200_456_458_802_993_406_410_049,
            162_259_276_829_213_363_391_578_010_288_127,
            162_259_276_829_213_363_391_578_010_287_957,
            162_259_276_829_213_363_391_578_010_287_051,
            1_298_074_214_633_706_907_132_624_082_304_889,
        ];

        for prime in test_primes.iter() {
            assert_eq!(is_odd_prime_factor(*prime), true, "{}", *prime);
        }
    }

    #[test]
    fn is_prime_factor_larger_primes_other() {
        let test_primes: [u128; 21] = [
            41_538_374_868_278_621_028_243_970_633_760_399,
            41_538_374_868_278_621_028_243_970_633_760_057,
            166_153_499_473_114_484_112_975_882_535_042_517,
            166_153_499_473_114_484_112_975_882_535_042_279,
            332_306_998_946_228_968_225_951_765_070_086_139,
            5_316_911_983_139_663_491_615_228_241_121_378_301,
            5_316_911_983_139_663_491_615_228_241_121_378_191,
            42_535_295_865_117_307_932_921_825_928_971_026_423,
            42_535_295_865_117_307_932_921_825_928_971_026_047,
            42_535_295_865_117_307_932_921_825_928_971_026_027,
            170_141_183_460_469_231_731_687_303_715_884_105_727,
            170_141_183_460_469_231_731_687_303_715_884_105_703,
            170_141_183_460_469_231_731_687_303_715_884_105_689,
            170_141_183_460_469_231_731_687_303_715_884_105_433,
            170_141_183_460_469_231_731_687_303_715_884_105_419,
            170_141_183_460_469_231_731_687_303_715_884_104_993,
            340_282_366_920_938_463_463_374_607_431_768_210_659,
            340_282_366_920_938_463_463_374_607_431_768_211_219,
            340_282_366_920_938_463_463_374_607_431_768_211_223,
            340_282_366_920_938_463_463_374_607_431_768_211_283,
            340_282_366_920_938_463_463_374_607_431_768_211_297,
        ];

        for prime in test_primes.iter() {
            assert_eq!(is_odd_prime_factor(*prime), true, "{}", *prime);
        }
    }

    #[test]
    fn is_prime_factor_large_composites() {
        let test_composites: [u128; 5] = [
            83_076_749_736_557_242_056_487_941_267_521_531,
            332_306_998_946_228_968_225_951_765_070_086_141,
            5_316_911_983_139_663_491_615_228_241_121_378_303,
            170_141_183_460_469_231_731_687_303_715_884_105_723,
            340_282_366_920_938_463_463_374_607_431_768_211_455,
        ];

        for comp in test_composites.iter() {
            assert_eq!(is_odd_prime_factor(*comp), false, "{}", *comp);
        }
    }

    #[test]
    fn is_prime_factor_range_containing_two_primes() {
        let start_num = (i128::MAX - 511) as u128;
        let stop_num = (i128::MAX - 505) as u128;

        let prime_count = (start_num..stop_num)
            .filter(|x| is_odd_prime_factor(*x))
            .count();

        assert_eq!(prime_count, 2);
    }

    #[test]
    fn is_prime_factor_range_containing_three_primes() {
        let start_num = u128::pow(2, 119) - 801;
        let stop_num = u128::pow(2, 119) - 744;

        let prime_count = (start_num..stop_num)
            .filter(|x| is_odd_prime_factor(*x))
            .count();

        assert_eq!(prime_count, 3);
    }

    #[test]
    fn is_prime_factor_range_containing_no_primes() {
        let start_num = u128::pow(2, 107) - 170;
        let stop_num = u128::pow(2, 107) - 1;

        // range is exclusive, the stop number is prime but not included here
        let prime_count = (start_num..stop_num)
            .filter(|x| is_odd_prime_factor(*x))
            .count();

        assert_eq!(prime_count, 0);
    }
}
