/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(any(feature = "std", all(feature = "alloc", no_std)))]
use crate::prelude::{Classifier, Identifier};
#[cfg(all(feature = "alloc", no_std))]
use alloc::string::String;

/// An appellation is considered to be a name or title that is used to identify an object.
/// For our purposes, an `Appellation` is a type that is used to identify an object in a computational space.
/// The appellation outlines a notation that combines an idenfifier, classifier, and name into a single unique
/// identifier for an object.
pub trait Appellation {
    type Class: Classifier;
    type Id: Identifier;

    fn class(&self) -> &Self::Class;

    fn id(&self) -> &Self::Id;

    fn name(&self) -> String;

    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }
}

/*
 ************* Implementations *************
*/
impl<Cls, Id, T> Appellation for (Cls, Id, T)
where
    Cls: Classifier,
    Id: Identifier,
    T: ToString,
{
    type Class = Cls;
    type Id = Id;

    fn class(&self) -> &Cls {
        &self.0
    }

    fn id(&self) -> &Id {
        &self.1
    }

    fn name(&self) -> String {
        self.2.to_string()
    }
}
