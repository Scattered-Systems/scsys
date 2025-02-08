/*
    Appellation: derive <test>
    Contrib: @FL03
*/
extern crate scsys_derive as scsys;

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::Getter,
    scsys::Set,
    scsys::With,
)]
pub struct Basic {
    pub key: String,
    pub weight: usize,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::Getter,
    scsys::Set,
    scsys::With,
)]
pub struct Generic<K, V> {
    pub key: K,
    pub weight: V,
}

#[derive(scsys::Getter, scsys::Params)]
pub struct LinearParams<T> {
    #[param]
    pub weight: T,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}

#[test]
fn test_gsw() {
    let mut data = Basic::default().with_key("key".to_string()).with_weight(1);

    assert_eq!(data.key(), "key");
    assert_eq!(*data.weight(), 1);

    data.set_key("new_key".to_string());
    data.set_weight(2);

    assert_eq!(data.key(), "new_key");
    assert_eq!(*data.weight(), 2);

    let data: Generic<&str, f64> = Generic::default().with_key("key").with_weight(1.00);

    assert_eq!(*data.key(), "key");
    assert_eq!(*data.weight(), 1.00);
}

#[test]
fn test_params() {
    let params = LinearParams { weight: 1.0 };

    assert_eq!(*params.weight(), 1.0);
    assert_eq!(LinearParamsKey::weight(), LinearParamsKey::Weight);
}

#[test]
fn test_variant_constructors() {
    assert_eq!(Something::a(), Something::A);
    assert_eq!(Something::b(1), Something::B(1));
    assert_eq!(Something::c(1, 2), Something::C { x: 1, y: 2 });
}
