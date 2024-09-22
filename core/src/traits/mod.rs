/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod adjust;
pub mod appellation;
pub mod classify;
pub mod dtype;
pub mod toggle;

pub mod ext {
    pub use self::slice::*;
    #[cfg(feature = "alloc")]
    pub use self::string::*;

    mod slice;
    mod string;

    pub(crate) mod prelude {}
}

pub(crate) mod prelude {
    pub use super::adjust::*;
    pub use super::appellation::*;
    pub use super::classify::*;
    pub use super::dtype::*;
    pub use super::ext::*;
    pub use super::toggle::*;
    pub use super::{IntoInner, Name};
}

/// [IntoInner] is typically used for basic structures that wrap a single value.
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

/// Interface for nameable data-structures
pub trait Name {
    fn name(&self) -> &str;
}
