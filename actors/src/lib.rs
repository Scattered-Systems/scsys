/*
    Appellation: actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
//!
//!
#[cfg(not(feature = "std"))]
extern crate alloc;

extern crate scsys_core as scsys;

pub use self::{direction::*, traits::*};

pub(crate) mod direction;
#[macro_use]
pub(crate) mod macros;

pub mod config;
pub mod messages;
pub mod power;
pub mod states;
pub mod traits;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::direction::*;
    pub use crate::messages::*;
    pub use crate::power::*;
    pub use crate::states::prelude::*;
    pub use crate::traits::prelude::*;
}
