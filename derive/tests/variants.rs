/*
    Appellation: derive <test>
    Contrib: @FL03
*/
extern crate scsys_derive as scsys;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}

#[test]
fn test_variant_constructors() {
    assert_eq!(Something::a(), Something::A);
    assert_eq!(Something::b(1), Something::B(1));
    assert_eq!(Something::c(1, 2), Something::C { x: 1, y: 2 });
}
