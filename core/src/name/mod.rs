/*
    Appellation: name <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Name
//!
//! This module works to implement various naming conventions and name-related primitives.

pub use self::casing::*;

pub(crate) mod casing;

pub(crate) mod prelude {
    pub use super::casing::CaseType;
}
