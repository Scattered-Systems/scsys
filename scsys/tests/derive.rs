/*
    Appellation: derive <test>
    Contrib: @FL03
*/
pub use impls::*;

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
fn test_variant_constructors() {
    assert_eq!(Something::a(), Something::A);
    assert_eq!(Something::b(1), Something::B(1));
    assert_eq!(Something::c(1, 2), Something::C { x: 1, y: 2 });
}

#[test]
fn test_wrapper() {
    let mut unit = Unit::<usize>::new(42);
    assert_eq!(unit.get(), &42);
    let prev = unit.replace(100);
    assert_eq!(prev, 42);
    assert_eq!(unit.get(), &100);
    let value = unit.take();
    assert_eq!(value, 100);
    assert_eq!(unit.get(), &0); // Default value for usize is 0
    // map the value
    let u2 = unit.map(|x| (x + 1) * 2);
    assert_eq!(u2.get(), &2);
}

mod impls {
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

    #[derive(
        Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::VariantConstructors,
    )]
    pub enum Something {
        A,
        B(usize),
        C { x: usize, y: usize },
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
        scsys::Wrapper,
        scsys::Display,
    )]
    #[cfg_attr(
        feature = "serde",
        derive(serde::Deserialize, serde::Serialize),
        serde(default, rename_all = "snake_case"),
        scsys(display(json))
    )]
    pub struct Unit<T>(pub T);
}
