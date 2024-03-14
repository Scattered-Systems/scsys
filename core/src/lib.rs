/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-core
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub mod errors;
pub mod id;
pub mod specs;
pub mod time;

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::primitives::*;

    pub use crate::utils::*;

    pub use crate::errors::*;
    pub use crate::id::*;
    pub use crate::specs::prelude::*;
    pub use crate::time::*;
}
