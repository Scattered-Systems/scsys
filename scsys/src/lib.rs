/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys
//!
//!
#[doc(inline)]
pub use scsys_core as core;

#[cfg(feature = "actors")]
#[doc(inline)]
pub use scsys_actors as actors;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;
#[cfg(feature = "stores")]
#[doc(inline)]
pub use scsys_stores as stores;

pub mod prelude {
    #[doc(inline)]
    pub use super::core::prelude::*;

    #[cfg(feature = "actors")]
    #[doc(inline)]
    pub use super::actors::prelude::*;
    #[cfg(feature = "stores")]
    #[doc(inline)]
    pub use crate::stores::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
}
