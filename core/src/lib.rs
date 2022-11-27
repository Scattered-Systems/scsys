/*
    Appellation: scsys-core <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{data::*, primitives::*, specs::*, utils::*};

pub mod accounts;
pub mod errors;
pub mod events;
pub mod extract;
pub mod loggers;
pub mod networking;
pub mod parse;
pub mod providers;

pub(crate) mod data;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;
