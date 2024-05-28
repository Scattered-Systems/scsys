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

pub use self::{traits::prelude::*, types::prelude::*};

#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::errors::{Error, Errors, Result};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::{primitives::*, utils::*};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod primitives;
#[macro_use]
pub(crate) mod seal;
#[cfg(any(feature = "std", feature = "alloc"))]
pub(crate) mod utils;

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod errors;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod hkt;
pub mod id;
#[doc(hidden)]
pub mod name;
pub mod stores;
pub mod sync;
#[cfg(feature = "std")]
pub mod time;
pub mod traits;
pub mod types;

pub mod prelude {
    pub use crate::id::prelude::*;
    pub use crate::name::prelude::*;
    pub use crate::stores::prelude::*;
    pub use crate::sync::prelude::*;
    #[cfg(feature = "std")]
    pub use crate::time::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub use crate::{errors::prelude::*, hkt::prelude::*, utils::*};
}
