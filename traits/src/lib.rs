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
pub(crate) mod seal;

pub mod adjust;
pub mod container;
pub mod convert;
pub mod dtype;
pub mod hkt;
pub mod named;
pub mod store;
pub mod string;
pub mod symbolic;
pub mod toggle;
pub mod wrapper;

#[doc(hidden)]
pub mod prelude {
    #[doc(inline)]
    pub use crate::adjust::*;
    #[doc(inline)]
    pub use crate::container::*;
    #[doc(inline)]
    pub use crate::convert::*;
    #[doc(inline)]
    pub use crate::dtype::*;
    #[doc(inline)]
    pub use crate::hkt::*;
    #[doc(inline)]
    pub use crate::named::*;
    #[doc(inline)]
    pub use crate::store::*;
    #[doc(inline)]
    pub use crate::string::*;
    #[doc(inline)]
    pub use crate::symbolic::*;
    #[doc(inline)]
    pub use crate::toggle::*;
    #[doc(inline)]
    pub use crate::wrapper::*;
}
