/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!
#[doc(inline)]
pub use self::{primitives::*, utils::*};

pub mod errors;
pub mod hkt;
pub mod id;
pub mod specs;
pub mod sync;
pub mod time;
pub mod types;

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::primitives::*;
    pub use crate::utils::*;

    pub use crate::errors::*;
    pub use crate::hkt::*;
    pub use crate::id::prelude::*;
    pub use crate::specs::prelude::*;
    pub use crate::time::*;
    pub use crate::types::*;
}
