/*
    Appellation: actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{direction::*, specs::*};

pub mod catalysts;
pub mod contexts;
pub mod events;
pub mod loggers;
pub mod messages;
pub mod states;

mod direction;
mod specs;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod prelude {
    pub use super::catalysts::*;
    pub use super::contexts::*;
    pub use super::direction::*;
    pub use super::loggers::*;
    pub use super::messages::*;
    pub use super::specs::*;
    pub use super::states::*;
}
