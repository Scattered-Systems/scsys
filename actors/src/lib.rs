/*
    Appellation: actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
//!
//!
extern crate scsys_core as scsys;

pub use self::{direction::*, specs::*};

pub mod catalysts;
pub mod contexts;
pub mod messages;
pub mod power;
pub mod states;

mod direction;
mod specs;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod prelude {
    pub use super::catalysts::*;
    pub use super::contexts::*;
    pub use super::direction::*;
    pub use super::messages::*;
    pub use super::power::*;
    pub use super::specs::*;
    pub use super::states::*;
}
