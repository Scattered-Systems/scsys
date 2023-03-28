/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{classify::*, identity::*, primitives::*, specs::*, timestamp::*, utils::*};

pub mod appellation;
pub mod errors;
pub mod extract;

mod classify;
mod identity;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod timestamp;
pub(crate) mod utils;
