/*
    Appellation: registries <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Registries
//!
//!
pub use self::registry::*;

pub(crate) mod registry;

pub trait Register {}

#[cfg(test)]
mod tests {}
