/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//!
//! Welcome to `scsys`, home to various primitives and utilities used throughout the [scsys.io](https://scsys.io) ecosystem.
//! The sdk is heavily feature gated, reducing its footprint and allowing for a more modular approach to development.
#![crate_name = "scsys"]
#![crate_type = "lib"]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;


#[doc(inline)]
#[cfg(feature = "traits")]
pub use scsys_traits::prelude::*;

#[doc(inline)]
pub use scsys_core::*;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use scsys_macros::*;
#[doc(inline)]
#[cfg(feature = "config")]
pub use scsys_config as config;
#[doc(inline)]
#[cfg(feature = "crypto")]
pub use scsys_crypto as crypto;
#[doc(inline)]
#[cfg(feature = "traits")]
pub use scsys_traits as traits;
#[doc(inline)]
#[cfg(feature = "utils")]
pub use scsys_util as utils;

pub mod prelude {
    #[doc(no_inline)]
    pub use scsys_core::prelude::*;

    #[cfg(feature = "config")]
    #[doc(no_inline)]
    pub use scsys_config::prelude::*;
    #[cfg(feature = "crypto")]
    #[doc(no_inline)]
    pub use scsys_crypto::prelude::*;
    #[cfg(feature = "derive")]
    #[doc(no_inline)]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    #[doc(no_inline)]
    pub use scsys_macros::*;
    #[cfg(feature = "traits")]
    #[doc(no_inline)]
    pub use scsys_traits::prelude::*;
}
