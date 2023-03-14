/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{ids::*, links::*, primitives::*, specs::*, timestamp::*, utils::*};

pub mod appellation;
pub mod errors;
pub mod extract;

pub(crate) mod ids;
pub(crate) mod links;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod timestamp;
pub(crate) mod utils;
