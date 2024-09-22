/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Errors
//!
//!
#![cfg(feature = "alloc")]
#[doc(inline)]
pub use self::{error::*, kinds::*};

pub(crate) mod error;
pub(crate) mod kinds;

/// A type alias for [core::result::Result] that employs the [crate::errors::Error] type
pub type Result<T = ()> = core::result::Result<T, Error>;

pub(crate) mod prelude {
    pub use super::error::Error;
    pub use super::kinds::*;
    pub use super::Result;
}
