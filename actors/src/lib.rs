/*
    Appellation: actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{direction::*, justify::*, specs::*};

pub mod agents;
pub mod catalysts;
pub mod contexts;
pub mod loggers;
pub mod messages;
pub mod states;

pub(crate) mod direction;
pub(crate) mod justify;
pub(crate) mod specs;

pub type Job = Box<dyn FnOnce() + Send + 'static>;
