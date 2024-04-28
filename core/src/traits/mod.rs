/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{appellation::*, classify::*, ext::prelude::*};

pub mod appellation;
pub mod classify;

pub mod ext {
    pub use self::prelude::*;

    pub mod slice;
    pub mod string;

    pub(crate) mod prelude {
        pub use super::slice::*;
        pub use super::string::*;
    
    }
}

/// Interface for data-structures that can be compared for equality
pub trait Contain<T>
where
    T: PartialEq,
{
    /// [Contain::contains] returns true if the given element is in the [Contain] instance
    fn contains(&self, elem: &T) -> bool;
    /// [Contain::contains_all] returns true if all elements in the given iterator are in the [Contain] instance
    fn contains_all(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().all(|i| self.contains(&i))
    }
    /// [Contain::contains_some] returns true if any element in the given iterator is in the [Contain] instance
    fn contains_some(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().any(|i| self.contains(&i))
    }
}

pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

/// Interface for nameable data-structures
pub trait Name {
    fn name(&self) -> String;

    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }
}

pub(crate) mod prelude {
    pub use super::appellation::*;
    pub use super::classify::*;
    pub use super::ext::prelude::*;
    pub use super::{Contain, Name};
}
