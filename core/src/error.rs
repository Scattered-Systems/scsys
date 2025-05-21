/*
    Appellation: error <module>
    Contrib: @FL03
*/
#[allow(unused_imports)]
#[doc(inline)]
pub use self::{core_error::*, raw_error::*, std_error::*};

/// this module implements an enumerated error type used throughout the sdk
mod core_error;
/// the this module implements a raw, generic error type wrapper
mod raw_error;
/// this module implements an alternative error type that uses some kind to distinguish
/// between different error types
mod std_error;

pub trait ErrorKind: core::fmt::Debug {}

impl ErrorKind for &str {}

#[cfg(feature = "alloc")]
impl ErrorKind for alloc::string::String {}
