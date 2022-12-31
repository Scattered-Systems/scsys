/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[cfg(feature = "actors")]
pub use scsys_actors as actors;
#[cfg(feature = "core")]
pub use scsys_core::*;
#[cfg(feature = "crypto")]
pub use scsys_crypto as crypto;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "gen")]
pub use scsys_gen as gen;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    #[cfg(feature = "actors")]
    pub use super::actors::{
        agents::*, catalysts::*, contexts::*, handlers::*, messages::*, sessions::*, states::*,
    };
    #[cfg(feature = "crypto")]
    pub use super::crypto::*;
    #[cfg(feature = "gen")]
    pub use super::gen::*;
    pub use super::*;
    #[cfg(feature = "core")]
    pub use super::{
        accounts::*, errors::*, events::*, extract::*, loggers::*, networking::*, providers::*,
    };
    // Extras
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;
}
