/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//!
//! Welcome to `scsys`, home to various primitives and utilities used throughout the [scsys.io](https://scsys.io) ecosystem.
//! The sdk is heavily feature gated, reducing its footprint and allowing for a more modular approach to development.
#[doc(inline)]
pub use scsys_core::*;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
#[doc(inline)]
pub use scsys_macros::*;
#[doc(inline)]
pub use scsys_traits::*;
#[cfg(feature = "utils")]
#[doc(inline)]
pub use scsys_util::*;

pub mod prelude {
    pub use scsys_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
    pub use scsys_traits::prelude::*;
}
