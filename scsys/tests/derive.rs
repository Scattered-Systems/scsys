/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(all(feature = "derive", test))]

use scsys::prelude::{FunctionalConstructors, Name, SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Name,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct TestStruct {
    timestamp: Timestamp,
}

#[derive(Clone, Copy, Debug, Default, Eq, FunctionalConstructors, Ord, PartialEq, PartialOrd)]
pub enum SampleUnit {
    #[default]
    A,
    B(usize),
    C {
        inner: usize,
    },
}

#[test]
fn test_serde_display() {
    let a = TestStruct::default();
    assert_eq!(a.name(), "TestStruct");
    let _string = a.to_string();
    assert!(true)
}

#[test]
fn test_functional_constructor() {
    assert_eq!(SampleUnit::a(), SampleUnit::A);
    assert_eq!(SampleUnit::b(0), SampleUnit::B(0));
    assert_eq!(SampleUnit::c(0), SampleUnit::C { inner: 0 });
}
