/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Classifier, Identifier};

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

pub trait FromAppellation<Cls, Id>
where
    Cls: Classifier,
    Id: Identifier,
{
    fn from_appellation(appellation: impl Appellation<Class = Cls, Id = Id>) -> Self;
}

pub trait TryFromAppellation<Cls, Id>
where
    Cls: Classifier,
    Id: Identifier,
    Self: Sized,
{
    type Error;
    fn try_from_appellation(
        appellation: impl Appellation<Class = Cls, Id = Id>,
    ) -> Result<Self, Self::Error>;
}

pub trait IntoAppellation<Cls, Id>
where
    Cls: Classifier,
    Id: Identifier,
{
    fn into_appellation(self) -> dyn Appellation<Class = Cls, Id = Id>;
}

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
