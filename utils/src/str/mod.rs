/*
    Appellation: str <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! string related utilities
//!
//! This module works to implement various naming conventions and name-related primitives.
#[doc(inline)]
pub use self::casing::*;

pub mod casing;

pub(crate) mod prelude {
    pub use super::casing::*;
}
