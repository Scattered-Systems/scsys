/*
    Appellation: scsys <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
#[cfg(feature = "core")]
pub use crate::{actors::*, components::*, core::*, data::*};
#[cfg(feature = "derive")]
pub use scsys_derive::*;
#[cfg(feature = "macros")]
pub use scsys_macros::*;

mod actors;
mod components;
mod core;
mod data;


pub mod prelude {
    #[cfg(feature = "bson")]
    pub use bson;
    #[cfg(feature = "chrono")]
    pub use chrono;
    #[cfg(feature = "config")]
    pub use config;
    #[cfg(feature = "rand")]
    pub use rand;

    #[cfg(feature = "core")]
    pub use super::{
        actors::{extract::*, generate::*, states::*}, 
        components::{clients::*, loggers::*, networking::*, providers::*},
        data::{handlers::*, models::*, schemas::*}
    };
}
