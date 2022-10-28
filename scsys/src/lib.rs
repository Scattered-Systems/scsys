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
#[cfg(feature = "crypto")]
pub use scsys_crypto as crypto;
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
        actors::{extract::*, generate::*, handlers::*, parse::*},
        components::{
            accounts::*, identities::*, logging::*, messages::*, networking::*, providers::*,
        },
        data::{caveats::*, models::*, schemas::*},
        states::*,
    };

    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*};
}
