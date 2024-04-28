/*
    Appellation: sync <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Synchronization
//!
//!

pub mod atomic;

pub(crate) mod prelude {
    pub use super::atomic::prelude::*;
}

#[cfg(test)]
mod tests {}
