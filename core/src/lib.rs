/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-core
#[doc(inline)]
pub use self::{primitives::*, specs::*, utils::*};

pub mod appellation;
pub mod channels;
pub mod classify;
pub mod errors;
pub mod extract;
pub mod identity;
pub mod time;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::utils::*;

    pub use super::appellation::*;
    pub use super::channels::*;
    pub use super::classify::*;
    pub use super::errors::*;
    pub use super::extract::*;
    pub use super::identity::*;
    pub use super::time::*;
}
