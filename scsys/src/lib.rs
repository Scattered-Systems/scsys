/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys

#[cfg(feature = "actors")]
pub use scsys_actors as actors;
#[cfg(feature = "core")]
pub use scsys_core as core;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;
#[cfg(feature = "stores")]
pub use scsys_stores as stores;

pub mod prelude {
    #[cfg(feature = "actors")]
    pub use super::actors::prelude::*;
    #[cfg(feature = "core")]
    pub use super::core::prelude::*;
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
    #[cfg(feature = "stores")]
    pub use crate::stores::prelude::*;
}
