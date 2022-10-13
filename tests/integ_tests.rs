//! Integration tests.
//!
use prime_factorization::Factorization;

#[test]
fn factorization_small_composite() {
    let num: u32 = 1027;

    let factor_repr = Factorization::run(num);

    assert_eq!(factor_repr.factors, vec![13, 79]);
    assert_eq!(factor_repr.is_prime, false);

    assert_eq!(factor_repr.num, num);
}

#[test]
fn factorization_u32_max() {
    let num = u32::MAX;
    let factor_repr = Factorization::run(num);

    assert_eq!(factor_repr.factors, vec![3, 5, 17, 257, 65_537]);
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_mid_composite() {
    let num: u64 = 5_298_573_603_985_982_111;

    let factor_repr = Factorization::run(num);

    assert_eq!(factor_repr.factors, vec![11, 3_057_227, 157_557_325_463]);
    assert_eq!(factor_repr.is_prime, false);

    assert_eq!(factor_repr.num, num);
}

#[test]
fn factorization_u64_max() {
    let num = u64::MAX;
    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![3, 5, 17, 257, 641, 65_537, 6_700_417]
    );
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_large_composite() {
    let num: u128 = u128::MAX - 4;

    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![169_909, 2_002_733_033_099_709_041_094_789_607_565_039]
    );
    assert_eq!(factor_repr.is_prime, false);

    assert_eq!(factor_repr.num, num);
}

#[test]
fn factorization_u128_max() {
    let num = u128::MAX;
    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![
            3,
            5,
            17,
            257,
            641,
            65_537,
            274_177,
            6_700_417,
            67_280_421_310_721
        ]
    );
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_multiple_factors() {
    let num: u128 = 1_742_252_654_625_863_814_028_009_129;

    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![11, 13, 7963, 13_831, 13_841, 19_991, 19_993, 19_997]
    );
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_small_prime() {
    let prime: u32 = 10_007;

    let factor_repr = Factorization::run(prime);

    assert_eq!(factor_repr.factors, vec![prime]);
    assert_eq!(factor_repr.is_prime, true);
}

#[test]
fn factorization_large_prime() {
    let prime: u128 = 837_598_237_598_275_982_352_383;

    let factor_repr = Factorization::run(prime);

    assert_eq!(factor_repr.factors, vec![prime]);
    assert_eq!(factor_repr.is_prime, true);
}

#[test]
fn factorization_product_of_prime_powers() {
    let num: u128 = 1_043_303_125_536_110_280_882_830_068_463_968_373;

    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![9941, 9941, 9941, 9941, 10_133, 10_133, 10_133, 10_133, 10_133]
    );
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_semiprime() {
    let num: u128 = 5_316_911_983_139_663_122_320_058_796_740_706_329;

    let factor_repr = Factorization::run(num);

    assert_eq!(
        factor_repr.factors,
        vec![72_057_594_037_927_931, 73_786_976_294_838_206_459]
    );
    assert_eq!(factor_repr.is_prime, false);
}

#[test]
fn factorization_for_one_and_zero() {
    let factor_repr = Factorization::run(0u32);

    assert_eq!(factor_repr.is_prime, false);
    assert_eq!(factor_repr.factors, vec![]);

    let factor_repr = Factorization::run(1u32);

    assert_eq!(factor_repr.is_prime, false);
    assert_eq!(factor_repr.factors, vec![]);
}

#[test]
fn factorization_readme_example() {
    let num: u128 = 3_746_238_285_234_848_709_827;

    let factor_repr = Factorization::run(num);

    assert_eq!(factor_repr.factors, vec![103_979, 36_028_797_018_963_913]);
}

#[test]
fn factorization_readme_other_example() {
    let num: u128 = 332_306_998_946_228_968_225_951_765_070_086_139;

    let factor_repr = Factorization::run(num);

    assert_eq!(factor_repr.is_prime, true);
    assert_eq!(factor_repr.factors, vec![num]);
}
