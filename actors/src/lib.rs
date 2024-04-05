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

pub use self::{direction::*, specs::*};

pub mod config;
pub mod messages;
pub mod power;
pub mod states;

pub(crate) mod direction;
pub(crate) mod specs;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::direction::*;
    pub use crate::messages::*;
    pub use crate::power::*;
    pub use crate::specs::*;
    pub use crate::states::*;
}
