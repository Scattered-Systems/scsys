/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//!
//! Welcome to `scsys`, home to various primitives and utilities used throughout the [scsys](https://scattered-systems.com) ecosystem.
//! The sdk is heavily feature gated, reducing its footprint and allowing for a more modular approach to development.
#[doc(inline)]
pub use scsys_core::*;

#[cfg(feature = "actors")]
#[doc(inline)]
pub use scsys_actors as actors;
#[cfg(feature = "derive")]
#[doc(inline)]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
#[doc(inline)]
pub use scsys_macros::*;
#[cfg(feature = "utils")]
#[doc(inline)]
pub use scsys_util::*;

// #66 - Cleanup the prelude module(s)
pub mod prelude {
    #[cfg(feature = "actors")]
    #[doc(inline)]
    pub use scsys_actors::prelude::*;
    #[doc(inline)]
    pub use scsys_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
}
