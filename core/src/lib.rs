/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This crate works to provide a set of utilities for working with state, time, and synchronization in Rust.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{state::State, types::prelude::*, utils::*};

#[doc(inline)]
pub use self::error::*;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

pub mod error;
pub mod id;
pub mod state;
pub mod time;
pub mod types;

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::id::prelude::*;
    #[doc(no_inline)]
    pub use crate::state::prelude::*;
    #[doc(no_inline)]
    pub use crate::time::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::*;
}
