/*
    Appellation: scsys-utils <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # `scsys-utils`
//!
//! utilities for the `scsys` ecosystem
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "std")]
pub mod project;
pub mod str;

pub mod prelude {
    #[doc(inline)]
    #[cfg(feature = "fs")]
    pub use crate::fs::*;
    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use crate::project::*;
    #[doc(inline)]
    pub use crate::str::prelude::*;
}
