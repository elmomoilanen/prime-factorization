use std::sync::{mpsc, Arc, Mutex};

use crate::{
    factor::{Factorization, MaybeFactors},
    UInt,
};

fn compare_arrays<T: UInt>(left_arr: &[T], right_arr: &[T]) {
    // right_arr can be larger as it might contain zero padding
    assert!(left_arr.len() <= right_arr.len());

    if right_arr.len() > left_arr.len() {
        // extra elements must be just zero padding
        assert_eq!(
            right_arr[left_arr.len()],
            T::zero(),
            "{}th element in correct sols array is not zero.",
            left_arr.len()
        );
    }

    for (elem_l, elem_r) in left_arr.iter().zip(right_arr.iter()) {
        assert_eq!(
            *elem_l, *elem_r,
            "left: {:?}, right: {:?}",
            left_arr, right_arr
        );
    }
}

fn compare_arrays_of_tuples<T: UInt, U: UInt>(left_arr: &[(T, U)], right_arr: &[(T, U)]) {
    // right_arr can be larger as it might contain zero padding
    assert!(left_arr.len() <= right_arr.len());

    if right_arr.len() > left_arr.len() {
        // extra elements must be just zero padding
        assert_eq!(
            right_arr[left_arr.len()],
            (T::zero(), U::zero()),
            "{}th element in correct sols array is not zero.",
            left_arr.len()
        );
    }

    for (elem_l, elem_r) in left_arr.iter().zip(right_arr.iter()) {
        assert_eq!(
            *elem_l, *elem_r,
            "left: {:?}, right: {:?}",
            left_arr, right_arr
        );
    }
}

#[test]
fn factorize_trial_primes() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    let test_integers: [u32; 8] = [2, 3, 5, 331, 4799, 7919, 7927, 7963];

    for int in test_integers.iter() {
        factorization.num = *int;
        factorization.factors.clear();

        let int_back = factorization.factorize_trial(*int);

        assert_eq!(int_back, 1); // factorization succeeded

        assert_eq!(factorization.factors.len(), 1);
        assert_eq!(factorization.factors[0], *int);
    }
}

#[test]
fn factorize_trial_small_composites() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    let test_num: [u32; 10] = [8, 10, 27, 150, 255, 1757, 47_514, 60_791, 83_521, 112_489];
    let correct_factors: [[u32; 4]; 10] = [
        [2, 2, 2, 0],
        [2, 5, 0, 0],
        [3, 3, 3, 0],
        [2, 3, 5, 5],
        [3, 5, 17, 0],
        [7, 251, 0, 0],
        [2, 3, 7919, 0],
        [31, 37, 53, 0],
        [17, 17, 17, 17],
        [13, 17, 509, 0],
    ];

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_facs) in it {
        factorization.num = *num;
        factorization.factors.clear();

        let num_back = factorization.factorize_trial(*num);

        assert_eq!(num_back, 1);

        compare_arrays(&factorization.factors, corr_facs);
    }
}

#[test]
fn factorize_trial_large_composites() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    let test_num: [u128; 7] = [
        39_049_078_408_188_253,
        337_364_201_967_782_238,
        1_521_827_844_866_817_193,
        3_827_567_052_006_943_601,
        32_589_158_477_190_044_730,
        105_736_595_740_338_477_810,
        2_699_302_794_582_910_996_440_074_437_437_656_779,
    ];
    let correct_factors: [[u128; 17]; 7] = [
        [
            223, 227, 229, 233, 239, 241, 251, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            2, 3, 3, 3, 3, 6113, 6599, 6599, 7823, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            151, 151, 151, 211, 211, 211, 211, 223, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            1997, 4051, 7583, 7879, 7919, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 0,
        ],
        [
            2, 3, 5, 7, 109, 137, 151, 157, 167, 181, 211, 223, 0, 0, 0, 0, 0,
        ],
        [
            139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139, 139,
        ],
    ];

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_facs) in it {
        factorization.num = *num;
        factorization.factors.clear();

        let num_back = factorization.factorize_trial(*num);

        assert_eq!(num_back, 1);

        compare_arrays(&factorization.factors, corr_facs);
    }
}

#[test]
fn factorize_fermat_composites() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    // [num, p1, p2]: num = p1 * p2
    let test_cases: [[u128; 3]; 6] = [
        [4087, 61, 67],
        [497_009, 701, 709],
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

    for case in test_cases.iter() {
        let num = case[0];

        factorization.num = num;
        factorization.factors.clear();

        let num_back = factorization.factorize_fermat(num, 2);

        assert_eq!(num_back, 1);

        compare_arrays(&factorization.factors, &case[1..]);
    }
}

#[test]
fn factorize_fermat_prime_powers() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    let test_cases: [[u128; 3]; 5] = [
        // [num, p, k] => num == p^k, where k is some power of two
        [6_806_881, 2609, 2],
        [9_555_603_847_167_361, 9887, 4],
        [416_997_623_116_370_028_124_580_469_121, 71, 16],
        [91_309_564_883_999_670_239_903_543_704_321, 9887, 8],
        [20_282_403_559_023_247_890_711_928_898_161, 67_108_859, 4],
    ];

    for case in test_cases.iter() {
        let num = case[0];

        factorization.num = num;
        factorization.factors.clear();

        let num_back = factorization.factorize_fermat(num, 2);

        assert_eq!(num_back, 1);

        let corr_factors = vec![case[1]; case[2].try_into().unwrap()];
        assert_eq!(factorization.factors.len(), corr_factors.len());

        compare_arrays(&factorization.factors, &corr_factors);
    }
}

#[test]
fn factorize_fermat_mix_composites() {
    let mut factorization = Factorization {
        num: 0,
        is_prime: false,
        factors: vec![],
    };

    let test_cases: [u128; 5] = [
        20_449,
        247_017_946_081,
        4_279_219_432_242_049,
        20_871_587_710_370_244_961,
        391_250_187_374_953_765_002_698_920_081,
    ];

    let correct_factors: [[u128; 8]; 5] = [
        [11, 11, 13, 13, 0, 0, 0, 0],
        [701, 701, 709, 709, 0, 0, 0, 0],
        [8087, 8087, 8089, 8089, 0, 0, 0, 0],
        [257, 257, 257, 257, 263, 263, 263, 263],
        [4999, 4999, 4999, 4999, 5003, 5003, 5003, 5003],
    ];

    let it = test_cases.iter().zip(correct_factors.iter());

    for (num, corr_facs) in it {
        factorization.num = *num;
        factorization.factors.clear();

        let num_back = factorization.factorize_fermat(*num, 2);

        assert_eq!(num_back, 1);

        factorization.factors.sort();

        compare_arrays(&factorization.factors, corr_facs);
    }
}

#[test]
fn wheel_factorization_as_runner() {
    let test_num = 51_384_281_238_756_235_937u128;

    let correct_factors: [u128; 5] = [7993, 8017, 8039, 8243, 12_101];
    let mut resulted_factors: Vec<u128> = vec![];

    let (tx, rx) = mpsc::channel();

    let maybe_factors = Arc::new(Mutex::new(MaybeFactors {
        num: test_num,
        factors: Vec::<(u128, bool)>::new(),
    }));

    let maybe_factors_cln = Arc::clone(&maybe_factors);

    Factorization::wheel_runner(maybe_factors_cln, test_num, tx);

    match rx.recv() {
        Ok(true) => {
            let data = maybe_factors.lock().unwrap();

            for tuple in (*data).factors.iter() {
                resulted_factors.push((*tuple).0);
            }
        }
        Ok(false) => panic!("wheel thread returned `false`."),
        Err(_) => panic!("wheel factorization error"),
    }

    assert_eq!(resulted_factors.len(), correct_factors.len());
    compare_arrays(&resulted_factors, &correct_factors);
}

#[test]
fn factorize_mid_composites_many_factors() {
    let test_num: [u64; 10] = [
        47_514,
        112_489,
        9_993_124,
        8_455_935_737,
        85_728_549_677,
        111_729_968_547,
        36_810_991_936_224_521,
        337_364_201_967_782_238,
        3_827_567_052_006_943_601,
        18_446_744_073_709_551_615,
    ];

    let correct_factors: [[u64; 9]; 10] = [
        [2, 3, 7919, 0, 0, 0, 0, 0, 0],
        [13, 17, 509, 0, 0, 0, 0, 0, 0],
        [2, 2, 359, 6959, 0, 0, 0, 0, 0],
        [181, 251, 373, 499, 0, 0, 0, 0, 0],
        [3299, 3299, 7877, 0, 0, 0, 0, 0, 0],
        [3, 17, 109, 1013, 19_841, 0, 0, 0, 0],
        [9791, 13_159, 16_903, 16_903, 0, 0, 0, 0, 0],
        [2, 3, 3, 3, 3, 6113, 6599, 6599, 7823],
        [1997, 4051, 7583, 7879, 7919, 0, 0, 0, 0],
        [3, 5, 17, 257, 641, 65_537, 6_700_417, 0, 0],
    ];

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_factors) in it {
        let factorization = Factorization::run(*num);

        // make sure that the integer remained correct
        assert_eq!(factorization.num, *num);

        compare_arrays(&factorization.factors, corr_factors);
    }
}

#[test]
fn factorize_semiprimes() {
    let test_num: [u128; 6] = [
        244_334_639,
        2_854_159_729_781,
        25_645_121_643_901_801,
        9_804_659_461_513_846_513,
        19_326_223_710_861_634_601,
        3_746_238_285_234_848_709_827,
    ];

    let correct_factors: [[u128; 2]; 6] = [
        [9199, 26_561],
        [718_433, 3_972_757],
        [5_394_769, 4_753_701_529],
        [4_641_991, 2_112_166_839_943],
        [3_267_000_013, 5_915_587_277],
        [103_979, 36_028_797_018_963_913],
    ];

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_factors) in it {
        let factorization = Factorization::run(*num);

        assert_eq!(factorization.num, *num);

        compare_arrays(&factorization.factors, corr_factors);
    }
}

#[test]
fn factorize_large_semiprimes() {
    let test_num: [u128; 5] = [
        2_776_889_953_055_853_600_532_696_901,
        90_124_258_835_295_998_242_413_094_252_351,
        7_060_005_655_815_754_299_976_961_394_452_809,
        2_082_064_493_491_567_088_228_629_031_592_644_077,
        340_282_366_920_938_463_463_374_607_431_768_211_451,
    ];

    let correct_factors: [[u128; 2]; 5] = [
        [11_560_410_863_851, 240_206_856_465_551],
        [18_812_497_391, 4_790_658_941_348_846_576_561],
        [6_988_699_669_998_001, 1_010_203_040_506_070_809],
        [434_609_209_084_157, 4_790_658_941_348_846_576_561],
        [169_909, 2_002_733_033_099_709_041_094_789_607_565_039],
    ];

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_factors) in it {
        let factorization = Factorization::run(*num);

        assert_eq!(factorization.num, *num);

        compare_arrays(&factorization.factors, corr_factors);
    }
}

#[test]
fn factorize_large_composites_many_factors() {
    let test_num: [u128; 6] = [
        9_898_235_283_523_592_353_852_353,
        340_282_366_920_938_463_463_374_607_431_768_211,
        252_458_274_525_971_054_424_244_242_423_424_245_235,
        340_282_366_920_938_463_463_374_607_431_768_211_441,
        340_282_366_920_938_463_463_374_607_431_768_211_450,
        340_282_366_920_938_463_463_374_607_431_768_211_455,
    ];

    let correct_factors: [[u128; 9]; 6] = [
        [3, 31_882_139, 103_487_779_197_453_809, 0, 0, 0, 0, 0, 0],
        [
            587,
            14_983,
            38_690_341_605_885_700_950_955_615_391,
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
            241,
            472_838_777,
            283_325_951_297,
            10_539_571_669_813_129,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            2,
            5,
            5,
            213_156_431,
            31_927_947_500_766_558_008_599_290_859,
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

    let it = test_num.iter().zip(correct_factors.iter());

    for (num, corr_factors) in it {
        let factorization = Factorization::run(*num);

        assert_eq!(factorization.num, *num);

        compare_arrays(&factorization.factors, corr_factors);
    }
}

#[test]
fn prime_factor_repr() {
    let test_num: [u128; 5] = [
        24_210_000,
        337_364_201_967_782_238,
        1_521_827_844_866_817_193,
        20_871_587_710_370_244_961,
        2_699_302_794_582_910_996_440_074_437_437_656_779,
    ];

    let correct_repr: [[(u128, u32); 5]; 5] = [
        [(2, 4), (3, 2), (5, 4), (269, 1), (0, 0)],
        [(2, 1), (3, 4), (6113, 1), (6599, 2), (7823, 1)],
        [(151, 3), (211, 4), (223, 1), (0, 0), (0, 0)],
        [(257, 4), (263, 4), (0, 0), (0, 0), (0, 0)],
        [(139, 17), (0, 0), (0, 0), (0, 0), (0, 0)],
    ];

    let it = test_num.iter().zip(correct_repr.iter());

    for (num, corr_repr) in it {
        let factorization = Factorization::run(*num);

        let factor_repr = factorization.prime_factor_repr();

        compare_arrays_of_tuples(&factor_repr, corr_repr);
    }
}
