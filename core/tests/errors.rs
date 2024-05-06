/*
    Appellation: errors <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate scsys_core as scsys;

use scsys::errors::{Error, ErrorKind};

#[test]
fn test_error() {
    let msg = "test";
    let err = Error::new(ErrorKind::custom("custom"), msg.to_string());
    assert_eq!(err.kind(), &ErrorKind::custom("custom"));
}

#[test]
#[cfg(feature = "serde")]
fn test_error_serde() {
    let err = Error::new(ErrorKind::custom("custom"), "test".to_string());
    let json = serde_json::to_value(&err).unwrap();
    let err2: Error = serde_json::from_value(json).unwrap();
    assert_eq!(err, err2);
}

