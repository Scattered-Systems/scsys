/*
    appellation: scsys-traits <library>
    authors: @FL03
*/
//! A collection of useful traits designed to be used throughout the ecosystem.
//!
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::prelude::*;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod cont;

pub mod convert;
pub mod dtype;
pub mod named;
pub mod string;
pub mod symbolic;
pub mod toggle;
pub mod wrapper;

pub mod ops {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod adjust;
    pub mod map;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::adjust::*;
        #[doc(inline)]
        pub use super::map::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use crate::cont::prelude::*;
    #[doc(inline)]
    pub use crate::ops::prelude::*;

    #[doc(inline)]
    pub use crate::convert::*;
    #[doc(inline)]
    pub use crate::dtype::*;
    #[doc(inline)]
    pub use crate::named::*;
    #[doc(inline)]
    pub use crate::string::*;
    #[doc(inline)]
    pub use crate::symbolic::*;
    #[doc(inline)]
    pub use crate::toggle::*;
    #[doc(inline)]
    pub use crate::wrapper::*;
}
