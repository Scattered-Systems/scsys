/*
    Appellation: scsys-stores <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # Stores
//!
//!

pub mod actions;
pub mod caches;
pub mod kv;
pub mod queues;
pub mod registries;
pub mod repos;
pub mod store;

pub mod prelude {
    pub use crate::actions::prelude::*;

    pub use crate::store::*;
}
