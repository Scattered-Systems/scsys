/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(test)]

use scsys_derive::{Display, VariantConstructors};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[display]
pub struct TestStruct {
    pub id: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, VariantConstructors)]
pub enum SampleUnit {
    #[default]
    A,
    B(usize),
    C {
        inner: usize,
    },
    AbcDef(usize),
}

#[test]
fn test_serde_display() {
    let test_struct = TestStruct::default();
    assert_eq!(
        test_struct.to_string(),
        serde_json::to_string(&test_struct).unwrap()
    );
}

#[test]
fn test_variant_constructors() {
    assert_eq!(SampleUnit::a(), SampleUnit::A);
    assert_eq!(SampleUnit::b(0), SampleUnit::B(0));
    assert_eq!(SampleUnit::c(0), SampleUnit::C { inner: 0 });
    assert_eq!(SampleUnit::abc_def(0), SampleUnit::AbcDef(0));
}
