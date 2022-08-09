use crate::arith::Arith;

#[test]
fn add_mod_small_type() {
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

        assert_eq!(u32::add_mod(x, y, modu), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_mod_mid_type_max_modu() {
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

        assert_eq!(u64::add_mod(x, y, modu), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_mod_large_type() {
    let modu = u16::MAX as u128;
    let u32max = u32::MAX as u128;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [5, 5, 10],
        [modu, modu + 1, 1],
        [modu + 1, modu, 1],
        [modu - 1, 2, 1],
        [2, modu - 1, 1],
        [u32max - 1, u32max - 1, modu - 2],
        [u32max + 1, u32max - 1, 0],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn add_mod_large_type_max_mod() {
    let modu = u128::MAX;

    // [x, y, res]: x + y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [modu, modu, 0],
        [modu - 1, modu, modu - 1],
        [modu, modu - 1, modu - 1],
        [modu - 1, modu - 1, modu - 2],
        [2, modu - 1, 1],
        [modu - 1, 2, 1],
        [0, modu - 2, modu - 2],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::add_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn sub_mod_small_type() {
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

        assert_eq!(u32::sub_mod(x, y, modu), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn sub_mod_large_type_max_modu() {
    let modu = u128::MAX;
    let u16max = u16::MAX as u128;

    // [x, y, res]: x - y = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [0, 0, 0],
        [modu, modu, 0],
        [modu, modu - 1, 1],
        [modu - 1, modu, modu - 1],
        [modu - 2, modu - 1, modu - 1],
        [1, modu - 1, 2],
        [modu - 1, 2, modu - 3],
        [u16max, u16max + 1, modu - 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::sub_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_mod_small_type() {
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

        assert_eq!(u32::mult_mod(x, y, modu), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_mod_large_type() {
    let modu = u32::MAX as u128;
    let u16max = u16::MAX as u128;

    // [x, y, res]: x * y = res (mod modu)
    let test_cases: [[u128; 3]; 10] = [
        [modu - 1, 0, 0],
        [1, modu - 1, modu - 1],
        [modu - 1, 1, modu - 1],
        [2, modu - 1, modu - 2],
        [modu - 1, 2, modu - 2],
        [modu - 1, modu - 1, 1],
        [u16max + 1, u16max + 1, 1],
        [u16max + 2, u16max + 1, u16max + 2],
        [1, u128::MAX, 0],
        [u128::MAX - 1, u128::MAX - 1, 1],
    ];

    for test in test_cases.iter() {
        let (x, y) = (test[0], test[1]);

        assert_eq!(u128::mult_mod(x, y, modu), test[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn mult_mod_large_type_max_modu() {
    let modu = u128::MAX;

    let test_cases: [[u128; 3]; 3] = [
        // [x, y, res]: x * y = res (mod modu)
        [modu - 1, modu - 1, 1],
        [modu - 1, 1, modu - 1],
        [modu - 2, modu - 1, 2],
    ];

    for case in test_cases.iter() {
        let (x, y) = (case[0], case[1]);

        assert_eq!(u128::mult_mod(x, y, modu), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn exp_mod_small_type() {
    let modu = 5;

    let test_cases: [[u32; 3]; 10] = [
        // [base, ex, res]: base^ex = res (mod modu)
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
        let (base, ex) = (case[0], case[1]);

        assert_eq!(
            u32::exp_mod(base, ex, modu),
            case[2],
            "b: {}, e: {}",
            base,
            ex
        );
    }
}

#[test]
fn exp_mod_mid_type_max_modu() {
    let modu = u64::MAX;

    let test_cases: [[u64; 3]; 3] = [
        // [base, ex, res]: base^ex = res (mod modu)
        [2, 1_000_000_000, 1],
        [modu - 1, 1_000_000_000, 1],
        [modu - 1, 1_000_000_001, modu - 1],
    ];

    for case in test_cases.iter() {
        let (base, ex) = (case[0], case[1]);

        assert_eq!(
            u64::exp_mod(base, ex, modu),
            case[2],
            "b: {}, e: {}",
            base,
            ex
        );
    }
}

#[test]
fn exp_mod_mid_type_large_modu_other() {
    let modu = i64::MAX as u64;

    let test_cases: [[u64; 3]; 3] = [
        // [base, ex, res]: base^ex = res (mod modu)
        [2, 9_999_999, 512],
        [9_987_654, 999_999_901_010_111, 2_940_910_929_841_963_431],
        [modu - 1, 100_000, 1],
    ];

    for case in test_cases.iter() {
        let (base, ex) = (case[0], case[1]);

        assert_eq!(
            u64::exp_mod(base, ex, modu),
            case[2],
            "b: {}, e: {}",
            base,
            ex
        );
    }
}

#[test]
fn exp_mod_large_type() {
    let modu = u64::MAX as u128;

    // [base, ex, res]: base^ex = res (mod modu)
    let test_cases: [[u128; 3]; 8] = [
        [2, 1_000_000_000, 1],
        [modu - 1, 1_000_000_000, 1],
        [modu - 1, 1_000_000_001, modu - 1],
        [2, 9_999_999_999_999, i64::MAX as u128 + 1],
        [modu - 1, modu - 1, 1],
        [modu - 1, modu - 2, modu - 1],
        [modu - 1, modu + 1, 1],
        [modu - 1, modu + 2, modu - 1],
    ];

    for test in test_cases.iter() {
        let (base, ex) = (test[0], test[1]);

        assert_eq!(
            u128::exp_mod(base, ex, modu),
            test[2],
            "b: {}, e: {}",
            base,
            ex
        );
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
        assert_eq!(u64::gcd_mod(x, y), case[2], "x: {}, y: {}", x, y);
    }
}

#[test]
fn gcd_equality() {
    let test_cases: [u128; 3] = [5, 16_358_049_139, u128::MAX];

    for case in test_cases.iter() {
        let x = *case;
        assert_eq!(u128::gcd_mod(x, x), x);
    }
}

#[test]
fn multip_inv_mid_type_exists() {
    let test_cases: [[u64; 3]; 10] = [
        // [a, m, x] s.t. a*x = 1 (mod m) is satisfied
        [0, 11, 0],
        [1, 11, 1],
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
        assert_eq!(u64::multip_inv(a, modu), case[2], "a: {}, mod: {}", a, modu);
    }
}

#[test]
fn multip_inv_mid_type_not_exists() {
    let test_cases: [[u64; 2]; 4] = [[5, 5000], [50, 5000], [55, 5000], [0, 5000]];

    for case in test_cases.iter() {
        let (a, modu) = (case[0], case[1]);
        assert_eq!(u64::multip_inv(a, modu), 0, "a: {}, mod: {}", a, modu);
    }
}

#[test]
fn multip_inv_large_type() {
    let u128max = u128::MAX;
    let i64max = i64::MAX as u128;

    // [x, modu, x^(-1)]: x * x^(-1) = 1 (mod modu)
    // if x^(-1) is zero, proper inverse doesn't exist
    let test_cases: [[u128; 3]; 10] = [
        [3, 5000, 1667],
        [1667, 5000, 3],
        [999, 5000, 3999],
        [55, 5000, 0],
        [999, i64max, 3_619_181_019_466_538_655],
        [i64max - 3, i64max, 3_074_457_345_618_258_602],
        [0, u128max, 0],
        [u128max, u128max, 0],
        [u128max - 1, u128max, u128max - 1],
        [
            2,
            u128max,
            170_141_183_460_469_231_731_687_303_715_884_105_728,
        ],
    ];

    for test in test_cases.iter() {
        let (x, modu) = (test[0], test[1]);

        assert_eq!(
            u128::multip_inv(x, modu),
            test[2],
            "x: {}, mod: {}",
            x,
            modu
        );
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

        assert_eq!(u32::jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
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

        assert_eq!(u128::jacobi_symbol(*x, *n), *res, "x: {}, n: {}", *x, *n);
    }
}

#[test]
fn trunc_square_mid_type() {
    let test_cases: [[u64; 2]; 5] = [
        [0, 0],
        [1, 1],
        [2, 4],
        [u32::MAX as u64, 18_446_744_065_119_617_025],
        [u32::MAX as u64 + 1, 0],
    ];

    for case in test_cases.iter() {
        assert_eq!(u64::trunc_square(case[0]), case[1]);
    }
}

#[test]
fn trunc_square_large_type() {
    let test_cases: [[u128; 2]; 4] = [
        [0, 0],
        [3, 9],
        [
            u64::MAX as u128,
            340_282_366_920_938_463_426_481_119_284_349_108_225,
        ],
        [u64::MAX as u128 + 1, 0],
    ];

    for case in test_cases.iter() {
        assert_eq!(u128::trunc_square(case[0]), case[1]);
    }
}
