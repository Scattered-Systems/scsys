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

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::error::*;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

#[cfg(feature = "alloc")]
pub mod error;
pub mod hkt;
pub mod id;
pub mod state;
#[doc(hidden)]
pub mod stores;
pub mod sync;
pub mod time;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod adjust;
    pub mod appellation;
    pub mod classify;
    pub mod convert;
    pub mod dtype;
    #[cfg(feature = "alloc")]
    pub mod string;
    pub mod toggle;
    pub mod wrapper;

    pub(crate) mod prelude {
        pub use super::adjust::*;
        pub use super::appellation::*;
        pub use super::classify::*;
        pub use super::convert::*;
        pub use super::dtype::*;
        #[cfg(feature = "alloc")]
        pub use super::string::*;
        pub use super::toggle::*;
        pub use super::wrapper::*;
    }
}
pub mod types;

pub mod prelude {
    pub use super::hkt::prelude::*;
    #[cfg(feature = "alloc")]
    pub use crate::error::*;
    pub use crate::id::prelude::*;
    pub use crate::state::prelude::*;
    #[doc(hidden)]
    pub use crate::stores::prelude::*;
    pub use crate::sync::prelude::*;
    pub use crate::time::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
}
