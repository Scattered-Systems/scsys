/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//! 
//! [![crates.io](https://img.shields.io/crates/v/scsys?style=for-the-badge&logo=rust)](https://crates.io/crates/scsys)
//! [![docs.rs](https://img.shields.io/docsrs/scsys?style=for-the-badge&logo=docs.rs)](https://docs.rs/scsys)
//! [![GitHub License](https://img.shields.io/github/license/scattered-systems/scsys?style=for-the-badge&logo=github)](https://github.com/scattered-systems/scsys/blob/main/LICENSE)
//!
//! ***
//! 
//! Welcome to `scsys`, this crate is tasked with providing a generic set of tools for the 
//! [`scsys.io`](https://scsys.io) ecosystem. It is designed to be a foundational library that 
//! helps establish a sense of consistency between individual efforts while working to reduce 
//! its overall footprint through modularization and feature-gating. These characteristics make
//! it suitable for use both within the ecosystem and outside of it.
//! 
#![crate_name = "scsys"]
#![crate_type = "lib"]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// the `config` module implements a set of standardized configuration schemas
#[doc(inline)]
#[cfg(feature = "config")]
pub use scsys_config as config;
#[doc(inline)]
pub use scsys_core::*;
/// cryptographic primitives and utilities implemented for the ecosystem
#[doc(inline)]
#[cfg(feature = "crypto")]
pub use scsys_crypto as crypto;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use scsys_macros::*;
/// this module contains various traits commonly used throughout the scsys ecosystem
#[doc(inline)]
#[cfg(feature = "traits")]
pub use scsys_traits as traits;
#[doc(inline)]
#[cfg(feature = "traits")]
pub use scsys_traits::prelude::*;
/// utilities for working with the scsys ecosystem
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
