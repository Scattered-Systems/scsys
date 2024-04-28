/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Classifier, Identifier};

/// An appellation is defined to be a name or title given to a person or thing; often being used in the context of wine.
/// Here, the `Appellation` is a unique object identifier that is composed of three independent parts:
///     - Classifier
///     - Identifier
///     - Name
///
/// While the name is typically referenced by external users, the classifier and id are used to navigate through computational space.
pub trait Appellation {
    type Class: Classifier;
    type Id: Identifier;

    fn class(&self) -> &Self::Class;

    fn id(&self) -> &Self::Id;

    fn name(&self) -> String;

    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }

    fn type_id(&self) -> core::any::TypeId
    where
        Self: Sized + 'static,
    {
        core::any::TypeId::of::<Self>()
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
