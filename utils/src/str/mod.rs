/*
    Appellation: str <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! string related utilities
//!
//! This module works to implement various naming conventions and name-related primitives.
#[doc(inline)]
pub use self::prelude::*;

#[cfg(feature = "alloc")]
pub(crate) mod impl_alloc;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod casing;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::casing::*;
    }
}

pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    pub use super::impl_alloc::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}

