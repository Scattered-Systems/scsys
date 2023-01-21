/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{misc::*, primitives::*, specs::*, utils::*};

pub mod errors;
pub mod extract;

pub(crate) mod misc;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;
