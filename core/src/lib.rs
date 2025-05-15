/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This crate works to provide a set of utilities for working with state, time, and synchronization in Rust.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{state::State, traits::prelude::*, types::prelude::*, utils::*};

#[doc(inline)]
pub use self::error::*;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

pub mod error;
pub mod hkt;
pub mod id;
pub mod state;
#[doc(hidden)]
pub mod stores;
pub mod time;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod adjust;
    pub mod appellation;
    pub mod classify;
    pub mod convert;
    pub mod dtype;
    pub mod string;
    pub mod toggle;
    pub mod wrapper;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::adjust::*;
        #[doc(inline)]
        pub use super::appellation::*;
        #[doc(inline)]
        pub use super::classify::*;
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::dtype::*;
        #[doc(inline)]
        pub use super::string::*;
        #[doc(inline)]
        pub use super::toggle::*;
        #[doc(inline)]
        pub use super::wrapper::*;
    }
}
pub mod types;

pub mod prelude {
    #[doc(no_inline)]
    pub use super::hkt::prelude::*;
    #[doc(no_inline)]
    #[cfg(feature = "alloc")]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::id::prelude::*;
    #[doc(no_inline)]
    pub use crate::state::prelude::*;
    #[doc(no_inline)]
    #[doc(hidden)]
    pub use crate::stores::prelude::*;
    #[doc(no_inline)]
    pub use crate::time::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::*;
}
