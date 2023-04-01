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
mod primitives;
mod specs;
mod timestamp;
mod utils;
