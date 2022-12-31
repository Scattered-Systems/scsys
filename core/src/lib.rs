/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{appellation::*, ids::*, links::*, primitives::*, specs::*, times::*, utils::*};

pub mod accounts;
pub mod errors;
pub mod events;
pub mod extract;
pub mod loggers;
pub mod networking;
pub mod providers;

pub(crate) mod appellation;
pub(crate) mod ids;
pub(crate) mod links;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod times;
pub(crate) mod utils;
