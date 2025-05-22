/*
    Appellation: error <module>
    Contrib: @FL03
*/
#[cfg(feature = "alloc")]
pub use self::std_error::StdError;
#[doc(inline)]
pub use self::{core_error::*, raw_error::*};

/// this module implements an enumerated error type used throughout the sdk
mod core_error;
/// the this module implements a raw, generic error type wrapper
mod raw_error;
/// this module implements an alternative error type that uses some kind to distinguish
/// between different error types
#[cfg(feature = "alloc")]
mod std_error;

/// this trait is used to denote various _error kinds_ for use throughout the sdk
pub trait ErrorKind {
    private!();
}

impl<T> ErrorKind for T
where
    T: AsRef<str>,
{
    seal!();
}
