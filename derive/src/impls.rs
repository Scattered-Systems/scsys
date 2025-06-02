/*
    appellation: impls <module>
    authors: @FL03
*/
//! this module contains the implementations of the derive macros for the `scsys` crate.
#[doc(inline)]
pub use self::prelude::*;

pub mod display;
pub mod getter;
pub mod variants;

pub(crate) mod prelude {
    pub use super::display::*;
    pub use super::getter::*;
    pub use super::variants::*;
}
