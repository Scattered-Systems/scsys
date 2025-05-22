use scsys_crypto::{H256, digest_to_hash};

#[test]
fn test_h256() {
    let a = H256::random();
    assert_ne!(a, H256::random());
    assert_eq!(a, digest_to_hash::<32>(a).into());
}

#[test]
fn test_h256_add() {
    let a = H256::random();
    let b = H256::random();
    assert_ne!(a, a + b);
    assert_eq!(a, a + 0f64);
}

#[test]
fn test_h256_div() {
    let a = H256::random();
    assert_eq!(a, a / 1f64);
    assert_ne!(a, a / 0f64);
    assert_ne!(a, a / 10f64)
}

#[test]
fn test_h256_mul() {
    let a = H256::random();
    let mut b = a;
    assert_eq!(a, a * 1f64);
    b *= 0f64;
    assert_ne!(a, b);
    assert_eq!(a * 0f64, b);
    assert_ne!(a, a * 10f64)
}
