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
pub mod messages;
pub mod states;

pub(crate) mod direction;
pub(crate) mod justify;

pub type Job = Box<dyn FnOnce() + Send + 'static>;
