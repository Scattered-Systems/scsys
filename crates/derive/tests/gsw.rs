/*
    Appellation: derive <test>
    Contrib: @FL03
*/
extern crate scsys_derive as scsys;

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

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::Get,
    scsys::GetMut,
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
    scsys::Get,
    scsys::GetMut,
    scsys::Set,
    scsys::With,
)]
pub struct Generic<K, V> {
    pub key: K,
    pub weight: V,
}
