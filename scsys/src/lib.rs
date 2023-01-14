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
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "gen")]
pub use scsys_gen as gen;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    #[cfg(feature = "actors")]
    pub use super::actors::{
        agents::*, catalysts::*, contexts::*, handlers::*, loggers::*, messages::*, networking::*,
        providers::*, sessions::*, states::*,
    };
    #[cfg(feature = "gen")]
    pub use super::gen::*;
    pub use super::*;
    #[cfg(feature = "core")]
    pub use super::{errors::*, extract::*};
    // Extras
    #[cfg(feature = "config")]
    pub use config;
}
