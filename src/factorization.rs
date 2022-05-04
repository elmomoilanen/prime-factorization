//! Implements integer factorization.
//!
//! The complete factorization algorithm consists of
//! - Trial division with small primes, see `small_primes` module for the array of these primes.
//! - Fermat's factorization method, useful if the integer is of the form n=(a+b)*(a-b).
//! - Primality test, see `primality` module for implementations of Miller-Rabin and strong Baillie-PSW tests.
//! - Lenstra elliptic-curve factorization with multiple of worker threads
//!
//! Constant `MAX_WORKERS` defines the max thread count, which is eight by default.
//! First thread will do wheel factorization targeting smaller prime factors and other threads
//! the actual elliptic-curve factorization method.
//!
//! Factorization algorithm stops when the factored number equals one.
//!
use std::cmp::Ordering;
use std::convert::{From, Into};
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::{Send, Sync};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use num::integer::{self, Roots};
use num::{PrimInt, Unsigned};
use rand::Rng;

pub trait UInt:
    PrimInt + Unsigned + Display + Debug + Roots + Send + Sync + From<u32> + Into<u128>
{
}

impl<T: PrimInt + Unsigned + Display + Debug + Roots + Send + Sync + From<u32> + Into<u128>> UInt
    for T
{
}

use crate::ladder_bytes as ladbytes;
use crate::small_primes as small_prm;
use crate::{arith, prime};

// max thread count for elliptic curve factorization
// MODIFY this if needed but set it at least to two
const MAX_WORKERS: usize = 8;

struct MaybeFactors<T: UInt> {
    num: T,
    factors: Vec<(T, bool)>,
}

pub struct Factorization<T: UInt> {
    num: T,
    factors: Vec<T>,
}

struct EllipticPoint<T: UInt>(T, T);

impl<T: UInt> Display for Factorization<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let flen = self.factors.len();

        match flen.cmp(&1) {
            Ordering::Less => write!(f, "Not factorized yet!"),
            Ordering::Equal => write!(f, "factors: {}", self.factors[0]),
            Ordering::Greater => {
                let k = self.factors.len() - 1;
                let str_prefix = self
                    .factors
                    .iter()
                    .take(k)
                    .fold(String::new(), |acc, &e| acc + &e.to_string() + ", ");

                write!(f, "factors: {}{}", str_prefix, self.factors[k])
            }
        }
    }
}

impl<T: 'static + UInt> Factorization<T> {
    pub fn new(num: T) -> Factorization<T> {
        if num <= T::one() {
            panic!("create failed: number must be at least two!");
        }
        Factorization {
            num,
            factors: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, num: T) {
        if num <= T::one() {
            panic!("update failed: number must be at least two!");
        }

        self.num = num;
        self.factors.clear();
    }

    pub fn run(&mut self) {
        let mut num = self.num;

        num = self.small_prime_division(num);

        loop {
            if num == T::one() {
                break;
            }

            num = self.fermats_factorization(num, 2);
            if num == T::one() {
                break;
            }

            if prime::is_odd_prime_factor(num) {
                self.factors.push(num);
                break;
            }

            num = self.elliptic_curve_factorization(num);
        }

        self.prune_duplicate_factors();
    }

    fn prune_duplicate_factors(&mut self) {
        self.factors.sort();

        let mut unique_factors: Vec<T> = vec![];
        let mut k = self.num;

        for factor in self.factors.iter().rev() {
            if k % *factor == T::zero() {
                unique_factors.push(*factor);
                k = k / *factor;
            }
        }

        unique_factors.reverse();
        self.factors = unique_factors;
    }

    fn small_prime_division(&mut self, mut num: T) -> T {
        for prm in small_prm::PRIMES.iter() {
            if num == T::one() {
                break;
            }
            let prime = (*prm).into();

            if num % prime == T::zero() {
                loop {
                    self.factors.push(prime);

                    num = num / prime;
                    if num % prime != T::zero() {
                        break;
                    }
                }
            }
        }

        num
    }

    fn fermats_factorization(&mut self, num: T, level: usize) -> T {
        let mut a = integer::sqrt(num);
        let mut a_square = arith::trunc_square(a);

        if a_square == num {
            if prime::is_odd_prime_factor(a) {
                for _ in 0..level {
                    self.factors.push(a);
                }
                return T::one();
            }
            let mut num_back = self.fermats_factorization(a, level << 1);

            if num_back > T::one() {
                // factoring not completed, return the original num
                num_back = num;
            }
            return num_back;
        }

        a = a + T::one();
        a_square = arith::trunc_square(a);

        if a_square == T::zero() {
            return num;
        }

        for _ in 0..10 {
            let b_square = a_square - num;
            let b = integer::sqrt(b_square);

            if arith::trunc_square(b) == b_square {
                let rounds = level >> 1;

                for _ in 0..rounds {
                    self.factors.push(a - b);
                    self.factors.push(a + b);
                }
                return T::one();
            }

            a = a + T::one();
            a_square = arith::trunc_square(a);

            if a_square == T::zero() {
                return num;
            }
        }

        num
    }

    fn elliptic_curve_factorization(&mut self, mut num: T) -> T {
        let mut ec_factors: Vec<(T, bool)> = Vec::new();
        num = self.ec_factorization(num, &mut ec_factors);

        for (ec_factor, is_sure_prime) in ec_factors {
            if is_sure_prime || prime::is_odd_prime_factor(ec_factor) {
                self.factors.push(ec_factor);
            } else {
                // factor is some power of a prime or multiple of several primes
                let mut factoriz = Factorization::new(ec_factor);
                factoriz.run();

                for new_factor in factoriz.factors {
                    self.factors.push(new_factor);
                }
            }
        }

        num
    }

    fn ec_factorization(&self, num: T, factors: &mut Vec<(T, bool)>) -> T {
        let (sender, receiver) = mpsc::channel();

        let factor_data = Arc::new(Mutex::new(MaybeFactors {
            num,
            factors: Vec::new(),
        }));

        for worker in 0..MAX_WORKERS {
            let sender = sender.clone();
            let factor_data = Arc::clone(&factor_data);

            thread::spawn(move || {
                if worker == 0 {
                    Factorization::wheel_factorize_worker(num, factor_data, sender);
                } else {
                    Factorization::ec_factorize_worker(num, factor_data, sender);
                }
            });
        }

        match receiver.recv() {
            Ok(completed) => {
                let fdata = factor_data.lock().unwrap();

                for tuple in (*fdata).factors.iter() {
                    factors.push(*tuple);
                }
                if completed {
                    T::one()
                } else {
                    (*fdata).num
                }
            }
            Err(_) => {
                panic!("all elliptic curve workers disconnected, unable to complete factorization.")
            }
        }
    }

    fn ec_factorize_worker(
        mut num: T,
        factor_data: Arc<Mutex<MaybeFactors<T>>>,
        sender: mpsc::Sender<bool>,
    ) {
        let one = T::one();
        let (mut curve, max_curves) = (1, 125);

        let mut p0 = EllipticPoint(one, one);

        loop {
            if num == one || curve > max_curves {
                break;
            }
            // Suyama's parametrization
            let sigma = rand::thread_rng().gen_range(6..u32::MAX).into();
            let (success, a) = Factorization::generate_elliptic_point(&mut p0, sigma, num);

            let factor = if success {
                let kp0 = Factorization::montgomery_ladder(&p0, a, num);
                arith::gcd(kp0.1, num)
            } else {
                a
            };

            if factor > one && factor < num {
                let mut fdata = factor_data.lock().unwrap();

                if factor > (*fdata).num {
                    num = (*fdata).num;
                } else {
                    num = num / factor;
                    (*fdata).num = num;
                    (*fdata).factors.push((factor, false));

                    if prime::is_odd_prime_factor(num) {
                        (*fdata).factors.push((num, true));
                        num = one;
                        (*fdata).num = num;
                    }
                }
            } else if factor == num && prime::is_odd_prime_factor(factor) {
                let mut fdata = factor_data.lock().unwrap();

                if factor == (*fdata).num {
                    num = one;
                    (*fdata).num = num;
                    (*fdata).factors.push((factor, true));
                } else {
                    num = (*fdata).num;
                }
            } else if curve & 31 == 0 {
                let fdata = factor_data.lock().unwrap();
                num = (*fdata).num;
            }

            curve += 1;
        }

        if sender.send(num == one).is_err() {}
    }

    fn generate_elliptic_point(mut p0: &mut EllipticPoint<T>, sigma: T, num: T) -> (bool, T) {
        let (three, four) = (3.into(), 4.into());

        let u = arith::mod_sub(arith::mod_mult(sigma, sigma, num), 5.into(), num);
        let u3 = arith::mod_exp(u, three, num);
        let v = arith::mod_mult(sigma, four, num);

        (*p0).0 = u3;
        (*p0).1 = arith::mod_exp(v, three, num);

        let (vu_diff, uv_add) = (
            arith::mod_exp(arith::mod_sub(v, u, num), three, num),
            arith::mod_add(arith::mod_mult(u, three, num), v, num),
        );
        let (a_numer, a_denumer) = (
            arith::mod_mult(vu_diff, uv_add, num),
            arith::mod_mult(arith::mod_mult(u3, four, num), v, num),
        );

        let a_denumer_inv = arith::multip_inv(a_denumer, num);

        if a_denumer_inv == T::zero() {
            // (a_denumer)^-1 doesn't exist
            return (false, arith::gcd(a_denumer, num));
        }

        let two = 2.into();

        let mut a = arith::mod_sub(arith::mod_mult(a_numer, a_denumer_inv, num), two, num);
        a = arith::mod_mult(
            arith::mod_add(a, two, num),
            arith::multip_inv(four, num),
            num,
        );

        (true, a)
    }

    fn montgomery_ladder(p0: &EllipticPoint<T>, a: T, num: T) -> EllipticPoint<T> {
        let mut q = EllipticPoint(p0.0, p0.1);
        let mut p = EllipticPoint(p0.0, p0.1);
        Factorization::elliptic_double(&mut p, a, num);

        let bits = u8::BITS;
        let msb_idx = u8::BITS - 1;
        let lastb_idx = ladbytes::KBYTES_10K_LEN - 1;

        for (i, byte) in ladbytes::KBYTES_10K.iter().enumerate() {
            for cbit in (0..bits).rev() {
                if (i == 0 && cbit == msb_idx) || (i == lastb_idx && cbit == 0) {
                    continue;
                }
                if (*byte >> cbit) & 1 == 1 {
                    Factorization::elliptic_add(&mut q, &p, p0, num);
                    Factorization::elliptic_double(&mut p, a, num);
                } else {
                    Factorization::elliptic_add(&mut p, &q, p0, num);
                    Factorization::elliptic_double(&mut q, a, num);
                }
            }
        }

        q
    }

    fn elliptic_double(point: &mut EllipticPoint<T>, a: T, num: T) {
        let psum = arith::mod_add(point.0, point.1, num);
        let psub = arith::mod_sub(point.0, point.1, num);

        let psum_square = arith::mod_mult(psum, psum, num);
        let psub_square = arith::mod_mult(psub, psub, num);

        let pmix = arith::mod_sub(psum_square, psub_square, num);

        point.0 = arith::mod_mult(psum_square, psub_square, num);
        point.1 = arith::mod_mult(
            pmix,
            arith::mod_add(psub_square, arith::mod_mult(a, pmix, num), num),
            num,
        );
    }

    fn elliptic_add(
        lp: &mut EllipticPoint<T>,
        rp: &EllipticPoint<T>,
        p0: &EllipticPoint<T>,
        num: T,
    ) {
        let lp_sum = arith::mod_add(lp.0, lp.1, num);
        let lp_sub = arith::mod_sub(lp.0, lp.1, num);

        let rp_sum = arith::mod_add(rp.0, rp.1, num);
        let rp_sub = arith::mod_sub(rp.0, rp.1, num);

        let lterm = arith::mod_mult(lp_sub, rp_sum, num);
        let rterm = arith::mod_mult(lp_sum, rp_sub, num);

        let term_add = arith::mod_add(lterm, rterm, num);
        lp.0 = arith::mod_mult(p0.1, arith::mod_mult(term_add, term_add, num), num);

        let term_sub = arith::mod_sub(lterm, rterm, num);
        lp.1 = arith::mod_mult(p0.0, arith::mod_mult(term_sub, term_sub, num), num);
    }

    fn wheel_factorize_worker(
        mut num: T,
        factor_data: Arc<Mutex<MaybeFactors<T>>>,
        sender: mpsc::Sender<bool>,
    ) {
        let wheel_inc: [u32; 48] = [
            2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6,
            2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
        ];

        let mut k = 7991.into(); // start from 1007th prime

        for wheel in wheel_inc.iter().cycle() {
            k = k + (*wheel).into();

            if k > num / k {
                break;
            }

            if num % k == T::zero() {
                let mut fdata = factor_data.lock().unwrap();

                if k > (*fdata).num || (*fdata).factors.iter().any(|&e| e.0 == k) {
                    num = (*fdata).num;
                    continue;
                }

                loop {
                    num = num / k;
                    (*fdata).num = num;
                    (*fdata).factors.push((k, true));

                    if num % k != T::zero() {
                        break;
                    }
                }
            }
        }

        if sender.send(num == T::one()).is_err() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_prime_division_for_primes() {
        let mut factor = Factorization::new(2u32);

        let test_integers: [u32; 8] = [2, 3, 5, 331, 4799, 7919, 7927, 7963];

        for int in test_integers.iter() {
            factor.update(*int);
            let int_back = factor.small_prime_division(*int);

            assert_eq!(int_back, 1); // factorization succeeded
            assert_eq!(factor.factors.len(), 1);
            assert_eq!(factor.factors[0], *int);
        }
    }

    fn compare_arrays<T: UInt>(left_arr: &[T], right_arr: &[T]) {
        // right_arr can be larger as it might contain zero padding
        assert!(left_arr.len() <= right_arr.len());

        for (elem_l, elem_r) in left_arr.iter().zip(right_arr.iter()) {
            assert_eq!(
                *elem_l, *elem_r,
                "left: {:?}, right: {:?}",
                left_arr, right_arr
            );
        }
    }

    #[test]
    fn small_prime_division_for_composites() {
        let mut factor = Factorization::new(2u32);

        let test_integers: [u32; 8] = [10, 27, 864, 1000, 47_514, 60_791, 83_521, 112_489];
        let correct_factors: [[u32; 8]; 8] = [
            [2, 5, 0, 0, 0, 0, 0, 0],
            [3, 3, 3, 0, 0, 0, 0, 0],
            [2, 2, 2, 2, 2, 3, 3, 3],
            [2, 2, 2, 5, 5, 5, 0, 0],
            [2, 3, 7919, 0, 0, 0, 0, 0],
            [31, 37, 53, 0, 0, 0, 0, 0],
            [17, 17, 17, 17, 0, 0, 0, 0],
            [13, 17, 509, 0, 0, 0, 0, 0],
        ];

        let it = test_integers.iter().zip(correct_factors.iter());

        for (int, corr_factors) in it {
            factor.update(*int);
            let int_back = factor.small_prime_division(*int);

            assert_eq!(int_back, 1);
            compare_arrays(&factor.factors, &corr_factors[..]);
        }
    }

    #[test]
    fn small_prime_division_for_large_composites() {
        let mut factor = Factorization::new(2u64);

        let test_integers: [u64; 3] = [
            85_728_549_677,
            337_364_201_967_782_238,
            3_827_567_052_006_943_601,
        ];
        let correct_factors: [[u64; 9]; 3] = [
            [3299, 3299, 7877, 0, 0, 0, 0, 0, 0],
            [2, 3, 3, 3, 3, 6113, 6599, 6599, 7823],
            [1997, 4051, 7583, 7879, 7919, 0, 0, 0, 0],
        ];

        let it = test_integers.iter().zip(correct_factors.iter());

        for (int, corr_factors) in it {
            factor.update(*int);
            let int_back = factor.small_prime_division(*int);

            assert_eq!(int_back, 1, "{}", *int);
            compare_arrays(&factor.factors, &corr_factors[..]);
        }
    }

    #[test]
    fn fermats_factorization() {
        let test_cases: [[u128; 3]; 5] = [
            [4087, 61, 67],
            [4_611_686_014_132_420_609, 2_147_483_647, 2_147_483_647],
            [1_070_271_221, 32_713, 32_717],
            [
                1_298_074_214_633_694_657_341_637_634_584_803,
                36_028_797_018_963_797,
                36_028_797_018_963_799,
            ],
            [
                5_316_911_983_139_663_487_003_542_222_693_990_401,
                2_305_843_009_213_693_951,
                2_305_843_009_213_693_951,
            ],
        ];

        let mut factor = Factorization::new(2u128);

        for case in test_cases.iter() {
            let num = case[0];
            factor.update(num);

            let num_back = factor.fermats_factorization(num, 2);

            assert_eq!(num_back, 1); // factorization succeeded
            assert_eq!(factor.factors.len(), 2);

            compare_arrays(&factor.factors, &case[1..]);
        }
    }

    #[test]
    fn fermats_factorization_prime_powers() {
        let test_cases: [[u128; 3]; 5] = [
            // [n, x, k] => n == x^k, where k is some power of two
            [6_806_881, 2609, 2],
            [9_555_603_847_167_361, 9887, 4],
            [416_997_623_116_370_028_124_580_469_121, 71, 16],
            [91_309_564_883_999_670_239_903_543_704_321, 9887, 8],
            [20_282_403_559_023_247_890_711_928_898_161, 67_108_859, 4],
        ];

        let mut factor = Factorization::new(2u128);

        for case in test_cases.iter() {
            let num = case[0];
            factor.update(num);

            let num_back = factor.fermats_factorization(num, 2);

            assert_eq!(num_back, 1); // factorization succeeded

            let corr_factors = vec![case[1]; case[2].try_into().unwrap()];
            assert_eq!(factor.factors.len(), corr_factors.len());

            compare_arrays(&factor.factors, &corr_factors[..]);
        }
    }

    #[test]
    fn fermats_factorization_mix_cases() {
        let test_integers: [u128; 3] = [
            20_449,
            4_279_219_432_242_049,
            391_250_187_374_953_765_002_698_920_081,
        ];

        let correct_factors: [[u128; 8]; 3] = [
            [11, 11, 13, 13, 0, 0, 0, 0],
            [8087, 8087, 8089, 8089, 0, 0, 0, 0],
            [4999, 4999, 4999, 4999, 5003, 5003, 5003, 5003],
        ];

        let mut factor = Factorization::new(2u128);
        let it = test_integers.iter().zip(correct_factors.iter());

        for (int, corr_facts) in it {
            factor.update(*int);

            let num_back = factor.fermats_factorization(*int, 2);

            assert_eq!(num_back, 1);

            factor.factors.sort();
            compare_arrays(&factor.factors, &corr_facts[..]);
        }
    }

    #[test]
    fn elliptic_double() {
        let modu = 29;
        let mut test_point = EllipticPoint::<u32>(11, 16);

        Factorization::elliptic_double(&mut test_point, 7, modu);

        assert_eq!(test_point.0, 13);
        assert_eq!(test_point.1, 10);
    }

    #[test]
    fn elliptic_add() {
        let modu = 29;

        let p0 = EllipticPoint::<u32>(11, 16);
        let mut left_point = EllipticPoint(p0.0, p0.1);
        let right_point = EllipticPoint::<u32>(13, 10);

        Factorization::elliptic_add(&mut left_point, &right_point, &p0, modu);

        assert_eq!(left_point.0, 23);
        assert_eq!(left_point.1, 17);
    }

    #[test]
    fn complete_factorization_two_factors() {
        let mut factor = Factorization::new(2u128);

        let test_integers: [u128; 5] = [
            2_854_159_729_781,
            25_645_121_643_901_801,
            9_804_659_461_513_846_513,
            19_326_223_710_861_634_601,
            3_746_238_285_234_848_709_827,
        ];

        let correct_factors: [[u128; 2]; 5] = [
            [718_433, 3_972_757],
            [5_394_769, 4_753_701_529],
            [4_641_991, 2_112_166_839_943],
            [3_267_000_013, 5_915_587_277],
            [103_979, 36_028_797_018_963_913],
        ];

        let it = test_integers.iter().zip(correct_factors.iter());

        for (int, corr_factors) in it {
            factor.update(*int);
            factor.run();

            compare_arrays(&factor.factors, &corr_factors[..]);
        }
    }

    #[test]
    fn complete_factorization_multiple_factors() {
        let mut factor = Factorization::new(2u128);

        let test_integers: [u128; 7] = [
            244_334_639,
            36_810_991_936_224_521,
            2_776_889_953_055_853_600_532_696_901,
            90_124_258_835_295_998_242_413_094_252_351,
            2_082_064_493_491_567_088_228_629_031_592_644_077,
            252_458_274_525_971_054_424_244_242_423_424_245_235,
            340_282_366_920_938_463_463_374_607_431_768_211_455,
        ];

        let correct_factors: [[u128; 9]; 7] = [
            [9199, 26_561, 0, 0, 0, 0, 0, 0, 0],
            [9791, 13_159, 16_903, 16_903, 0, 0, 0, 0, 0],
            [11_560_410_863_851, 240_206_856_465_551, 0, 0, 0, 0, 0, 0, 0],
            [
                18_812_497_391,
                4_790_658_941_348_846_576_561,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                434_609_209_084_157,
                4_790_658_941_348_846_576_561,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                5,
                416_797,
                589_360_206_969_257,
                205_548_452_538_501_643,
                0,
                0,
                0,
                0,
                0,
            ],
            [
                3,
                5,
                17,
                257,
                641,
                65_537,
                274_177,
                6_700_417,
                67_280_421_310_721,
            ],
        ];

        let it = test_integers.iter().zip(correct_factors.iter());

        for (int, corr_factors) in it {
            factor.update(*int);
            factor.run();

            compare_arrays(&factor.factors, &corr_factors[..]);
        }
    }
}
