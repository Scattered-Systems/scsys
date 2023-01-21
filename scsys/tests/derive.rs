/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:  ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
use scsys::{SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Default, SerdeDisplay, Deserialize, Serialize)]
pub struct TestStruct {
    timestamp: Timestamp,
}

#[derive(Default)]
struct Pancakes;

#[test]
fn test_hashable_derive() {
    let a = TestStruct::default();
    let _string = a.to_string();
    assert!(true)
}
