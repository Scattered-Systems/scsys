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
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    #[cfg(feature = "agents")]
    pub use super::agents::{self, catalysts::*, connect::*, handlers::*, justify::*, messages::*};
    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*, keys::*};
    pub use super::{
        accounts::*, addresses::*, errors::*, events::*, extract::*, gen::*, identities::*,
        loggers::*, networking::*, parse::*, providers::*, stamps::*,
    };
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
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
}
