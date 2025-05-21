use scsys_crypto::{H256, digest_to_hash};

#[test]
fn test_h256() {
    let a = H256::generate();
    assert_ne!(a, H256::generate());
    assert_eq!(a, digest_to_hash::<32>(a).into());
}

// #[test]
// fn test_concat() {
//     let mut a = H256::generate();
//     let b = H256::generate();
//     let expected: H256 = concat_b3(a.into(), Some(b.into())).into();
//     assert_eq!(a.concat(&b), expected);
// }

#[test]
fn test_addition() {
    let a = H256::generate();
    let b = H256::generate();
    assert_ne!(a, a + b);
    assert_eq!(a, a + 0f64);
}

#[test]
fn test_division() {
    let a = H256::generate();
    assert_eq!(a, a / 1f64);
    assert_ne!(a, a / 0f64);
    assert_ne!(a, a / 10f64)
}

#[test]
fn test_multiplication() {
    let a = H256::generate();
    let mut b = a;
    assert_eq!(a, a * 1f64);
    b *= 0f64;
    assert_ne!(a, b);
    assert_eq!(a * 0f64, b);
    assert_ne!(a, a * 10f64)
}
