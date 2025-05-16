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
#[doc(inline)]
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use scsys_macros::*;
#[doc(inline)]
#[cfg(feature = "traits")]
pub use scsys_traits::*;
#[doc(inline)]
#[cfg(feature = "utils")]
pub use scsys_util::*;

#[doc(inline)]    
#[cfg(feature = "config")]
pub use scsys_cnf as cnf;

pub mod prelude {
    pub use scsys_core::prelude::*;
    #[cfg(feature = "config")]
    pub use scsys_cnf::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
    #[cfg(feature = "traits")]
    pub use scsys_traits::prelude::*;
}
