/*
    Appellation: scsys-cnf <library>
    Contrib: @FL03
*/
//! A collection of common configuration primitives and utilities used throughout the
//! [scsys-io](https://scsys.io) ecosystem.
//!

#[doc(inline)]
pub use self::types::prelude::*;

#[doc(no_inline)]
#[cfg(feature = "config")]
pub use config;

#[macro_use]
pub(crate) mod seal;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod endpoint;
    pub mod mode;
    pub mod scope;
    pub mod workspace;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::endpoint::*;
        #[doc(inline)]
        pub use super::mode::*;
        #[doc(inline)]
        pub use super::scope::*;
        #[doc(inline)]
        pub use super::workspace::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use super::types::prelude::*;
}
