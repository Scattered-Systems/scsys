/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        An appellation is defined to be a name or title given to a person or thing.
        For our purposes, an appellation describes a novel naming schematic used to accelaerate the process of identifying persistent objects hosted on a multi-layered peer-to-peer network.

        An appellation is composed of three parts: a classifier, an identifier, and a name.
            The class is a classifier that describes the type of object being identified.
            The id is a unique identifier that distinguishes the object from all other objects of the same class.
            The name is a human-readable string that is used to identify the object in a human-readable context.
*/
use crate::{Classifier, Identifier};

pub trait Appellation<Cls, Id>
where
    Cls: Classifier,
    Id: Identifier,
{
    fn class(&self) -> &Cls;
    fn id(&self) -> &Id;
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }
}

impl<Cls, Id, T> Appellation<Cls, Id> for (Cls, Id, T)
where
    Cls: Classifier,
    Id: Identifier,
    T: ToString,
{
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
