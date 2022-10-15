/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#![allow(unused_imports)]

#[doc(inline)]
#[cfg(feature = "core")]
pub use scsys_core::*;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    #[cfg(feature = "anyhow")]
    pub use anyhow;
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;
    #[cfg(feature = "rand")]
    pub use rand;
    #[cfg(feature = "ring")]
    pub use ring;

    #[cfg(feature = "core")]
    pub use super::{
        actors::{extract::*, generate::*},
        components::{accounts::*, identities::*, logging::*, networking::*, providers::*},
        data::{models::*, schemas::*},
        crypto::*,
        states::*
    };
}
