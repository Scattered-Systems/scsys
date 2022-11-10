/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub mod actors;
pub mod components;
pub mod data;

#[doc(inline)]
#[cfg(feature = "core")]
pub use scsys_core as core;
#[cfg(feature = "crypto")]
pub use scsys_crypto as crypto;
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

pub trait Named {
    fn name() -> String;
}

pub mod prelude {
    #[doc(inline)]
    pub use bson;
    #[doc(inline)]
    pub use chrono;
    #[doc(inline)]
    pub use config;
    #[doc(inline)]
    pub use log;
    #[doc(inline)]
    pub use rand;
    #[doc(inline)]
    pub use ring;

    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "core")]
    pub use super::core::{self, contexts::*, errors::*, events::*, states::*};

    #[cfg(feature = "crypto")]
    pub use super::crypto::{self, hash::*, keys::*};
    #[cfg(feature = "derive")]
    pub use scsys_derive::*;
}
