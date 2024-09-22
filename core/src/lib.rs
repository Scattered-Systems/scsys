/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::{traits::prelude::*, types::prelude::*, utils::*};

#[cfg(feature = "alloc")]
pub use self::errors::{Error, Errors, Result};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

#[cfg(feature = "alloc")]
pub mod errors;
pub mod hkt;
pub mod id;
#[doc(hidden)]
pub mod name;
#[doc(hidden)]
pub mod stores;
pub mod sync;
pub mod time;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use super::hkt::prelude::*;
    #[cfg(feature = "alloc")]
    pub use crate::errors::prelude::*;
    pub use crate::id::prelude::*;
    #[doc(hidden)]
    pub use crate::name::prelude::*;
    #[doc(hidden)]
    pub use crate::stores::prelude::*;
    pub use crate::sync::prelude::*;
    pub use crate::time::prelude::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
}
