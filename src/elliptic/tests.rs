use crate::elliptic::EllipticCurve;

#[test]
fn elliptic_add_small_type() {
    let modu = 29;

    let p0 = EllipticCurve::<u32> { x: 11, z: 16 };
    let mut p = EllipticCurve { x: p0.x, z: p0.z };
    let q = EllipticCurve::<u32> { x: 13, z: 10 };

    p.elliptic_add(&q, &p0, modu);

    assert_eq!(p.x, 23);
    assert_eq!(p.z, 17);
}

#[test]
fn elliptic_add_mid_type() {
    let modu = 29;

    let p0 = EllipticCurve::<u64> { x: 11, z: 16 };
    let mut p = EllipticCurve { x: p0.x, z: p0.z };
    let q = EllipticCurve::<u64> { x: 13, z: 10 };

    p.elliptic_add(&q, &p0, modu);

    assert_eq!(p.x, 23);
    assert_eq!(p.z, 17);
}

#[test]
fn elliptic_double_small_type() {
    let modu = 29;

    let mut p = EllipticCurve::<u32> { x: 11, z: 16 };

    p.elliptic_double(7, modu);

    assert_eq!(p.x, 13);
    assert_eq!(p.z, 10);
}

#[test]
fn elliptic_double_mid_type() {
    let modu = 29;

    let mut p = EllipticCurve::<u64> { x: 11, z: 16 };

    p.elliptic_double(7, modu);

    assert_eq!(p.x, 13);
    assert_eq!(p.z, 10);
}
