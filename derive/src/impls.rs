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
pub mod wrapper;

#[allow(unused_imports)]
pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::display::*;
    #[doc(inline)]
    pub use super::getter::*;
    #[doc(inline)]
    pub use super::variants::*;
    #[doc(inline)]
    pub use super::wrapper::*;
}
