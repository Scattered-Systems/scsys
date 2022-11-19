/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[cfg(feature = "agents")]
pub use scsys_agents as agents;
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
    #[cfg(feature = "agents")]
    pub use super::agents::{self, catalysts::*, connect::*, handlers::*, justify::*, messages::*};
    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*, keys::*};
    #[cfg(feature = "gen")]
    pub use super::gen::*;
    #[cfg(feature = "core")]
    pub use super::{
        accounts::*, addresses::*, errors::*, events::*, extract::*, identities::*, loggers::*,
        networking::*, parse::*, providers::*, stamps::*,
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
