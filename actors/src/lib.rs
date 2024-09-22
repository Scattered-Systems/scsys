/*
    Appellation: actors <library>
    Creator: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
//!
//! This library seeks to provide a suite of tools for creating and managing actors in Rust.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate scsys_core as scsys;

pub use self::traits::*;

pub(crate) mod actor;
#[macro_use]
pub(crate) mod macros;

pub mod messages;
pub mod power;
pub mod traits;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub mod prelude {
    pub use crate::actor::*;
    pub use crate::messages::*;
    pub use crate::power::*;
    pub use crate::traits::prelude::*;
}
