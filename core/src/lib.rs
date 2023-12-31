/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-core
#[doc(inline)]
pub use self::{primitives::*, specs::*, utils::*};

pub mod appellation;
pub mod classify;
pub mod errors;
pub mod extract;
pub mod id;
pub mod time;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::primitives::*;
    pub use crate::specs::*;
    pub use crate::utils::*;

    pub use crate::appellation::*;
    pub use crate::classify::*;
    pub use crate::errors::*;
    pub use crate::extract::*;
    pub use crate::id::*;
    pub use crate::time::*;
}
