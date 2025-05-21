//! A collection of useful traits designed to be used throughout the ecosystem.
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::prelude::*;

#[macro_use]
pub(crate) mod seal;

pub mod adjust;
pub mod convert;
pub mod dtype;
pub mod hkt;
pub mod named;
pub mod store;
pub mod string;
pub mod symbolic;
pub mod toggle;
pub mod wrapper;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::adjust::*;
    #[doc(no_inline)]
    pub use crate::convert::*;
    #[doc(no_inline)]
    pub use crate::dtype::*;
    #[doc(no_inline)]
    pub use crate::hkt::*;
    #[doc(no_inline)]
    pub use crate::named::*;
    #[doc(no_inline)]
    pub use crate::store::*;
    #[doc(no_inline)]
    pub use crate::string::*;
    #[doc(no_inline)]
    pub use crate::symbolic::*;
    #[doc(no_inline)]
    pub use crate::toggle::*;
    #[doc(no_inline)]
    pub use crate::wrapper::*;
}
