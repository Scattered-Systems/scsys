/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::prelude::*;

pub mod appellation;
pub mod classify;

pub mod ext {
    pub use self::prelude::*;

    pub(crate) mod slice;
    pub(crate) mod string;

    pub(crate) mod prelude {
        pub use super::slice::*;
        #[cfg(any(feature = "alloc", feature = "std"))]
        pub use super::string::*;
    }
}

/// [IntoInner] is typically used for basic structures that wrap a single value.
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

/// Interface for nameable data-structures
#[cfg(any(feature = "std", all(feature = "alloc", no_std)))]
pub trait Name {
    fn name(&self) -> &str;

    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }
}

pub(crate) mod prelude {
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub use super::appellation::*;
    pub use super::classify::*;
    pub use super::ext::prelude::*;
    pub use super::IntoInner;
    #[cfg(any(feature = "alloc", feature = "std"))]
    pub use super::Name;
}
