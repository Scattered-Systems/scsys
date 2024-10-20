/*
    Appellation: scsys-utils <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Utils
//!

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::str::*;

pub mod str;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::str::prelude::*;
}
