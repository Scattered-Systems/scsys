/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::specs::*;

pub(crate) mod specs;

#[cfg(feature = "actors")]
pub use scsys_actors as actors;
#[cfg(feature = "core")]
pub use scsys_core as core;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub mod prelude {
    pub use super::*;

    #[cfg(feature = "actors")]
    pub use super::actors::{
        agents::*, catalysts::*, contexts::*, handlers::*, loggers::*, messages::*, states::*,
    };

    #[cfg(feature = "core")]
    pub use super::core::{errors::*, extract::*};
    // Extras
    #[cfg(feature = "config")]
    pub use config;
}
