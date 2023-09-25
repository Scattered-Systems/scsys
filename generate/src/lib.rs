/*
    Appellation: scsys-generate <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-generate

pub use self::{primitives::*, specs::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::utils::*;
}
