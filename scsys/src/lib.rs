/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
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
    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*, keys::*};
    #[cfg(feature = "gen")]
    pub use super::gen::*;
    #[cfg(feature = "core")]
    pub use super::{
        accounts::*, agents::*, catalysts::*, contexts::*, errors::*, events::*, extract::*,
        handlers::*, justify::*, loggers::*, messages::*, networking::*, parse::*, providers::*,
        states::*,
    };
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
    #[cfg(feature = "macros")]
    pub use scsys_macros::*;
    // Extras
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;
}
