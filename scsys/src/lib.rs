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
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
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

    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*, keys::*};
    pub use super::{
        accounts::*, addresses::*, agents::*, contexts::*, 
        errors::*, events::*, extract::*, gen::*, handlers::*, 
        identities::*, logging::*, networking::*, parse::*, 
        providers::*, stamps::*, states::*
    };
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
}
