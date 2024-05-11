/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{appellation::*, classify::*, ext::prelude::*};

pub mod appellation;
pub mod classify;

pub mod ext {
    pub use self::prelude::*;

    pub(crate) mod slice;
    pub(crate) mod string;

    pub(crate) mod prelude {
        pub use super::slice::*;
        pub use super::string::*;
    }
}

/// [IntoInner] is typically used for basic structures that wrap a single value.
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
    pub use super::{IntoInner, Name};
}
