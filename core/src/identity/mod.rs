/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # identity
pub use self::{builder::*, ids::*};

mod builder;
mod ids;

pub type BoxedId = Box<dyn Identifier>;

/// Interface for identifiable data-structures
pub trait Identifiable {
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
}

impl<Id: Identifier> Identifiable for Id {
    type Id = Id;

    fn id(&self) -> &Self::Id {
        self
    }
}

/// Interface for identifier data-structures
pub trait Identifier: ToString {}

impl<T> Identifier for T where T: ToString {}
