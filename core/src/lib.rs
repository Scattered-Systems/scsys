/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Core
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[doc(inline)]
pub use self::{traits::prelude::*, types::prelude::*, utils::*};

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;
pub(crate) mod utils;

pub mod errors;
pub mod hkt;
pub mod id;
pub mod sync;
pub mod time;
pub mod traits;
pub mod types;

///
pub const DEFAULT_IGNORE_CHARS: &[char] = &['[', ']', ',', '.', ' '];

pub mod prelude {
    pub use crate::DEFAULT_IGNORE_CHARS;

    pub use crate::errors::*;
    pub use crate::hkt::prelude::*;
    pub use crate::id::prelude::*;
    pub use crate::sync::prelude::*;
    pub use crate::time::*;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
}
