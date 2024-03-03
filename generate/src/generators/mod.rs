/*
    Appellation: generators <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Generators
pub use self::generator::*;

pub(crate) mod generator;

pub trait Generative {}

impl Generative for Vec<u8> {}

impl Generative for String {}

#[cfg(test)]
mod tests {}
