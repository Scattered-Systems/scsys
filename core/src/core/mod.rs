/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{primitives::*, specs::*, utils::*};

pub mod errors;
pub mod states;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;
