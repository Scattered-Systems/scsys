//! A collection of useful traits designed to be used throughout the ecosystem.
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::symbolic::*;

#[macro_use]
pub(crate) mod seal;

pub mod symbolic;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::symbolic::*;
}
