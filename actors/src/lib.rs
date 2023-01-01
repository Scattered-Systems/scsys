/*
    Appellation: scsys-actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{direction::*, justify::*};

pub mod agents;
pub mod catalysts;
pub mod contexts;
pub mod handlers;
pub mod loggers;
pub mod messages;
pub mod networking;
pub mod providers;
pub mod sessions;
pub mod states;

pub(crate) mod direction;
pub(crate) mod justify;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub trait Temporal {}
