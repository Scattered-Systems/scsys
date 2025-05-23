/*
    Appellation: scsys-utils <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # `scsys-utils`
//! 
//! utilities for the `scsys` ecosystem
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "fs")]
pub mod fs;
pub mod project;
pub mod str;

#[allow(unused_imports)]
pub mod prelude {
    #[cfg(feature = "fs")]
    #[doc(inline)]
    pub use crate::fs::*;
    #[doc(inline)]
    pub use crate::project::*;
    #[doc(inline)]
    pub use crate::str::prelude::*;
}
