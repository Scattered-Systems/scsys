/*
    appellation: cont <module>
    authors: @FL03
*/
//! this module implements a set of traits used to establish a common interface for containers
//! and other generic storage types.
#[doc(inline)]
pub use self::prelude::*;

pub mod container;
pub mod entry;
pub mod get;
pub mod hkt;
pub mod store;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::container::*;
    #[doc(inline)]
    pub use super::get::*;
    #[doc(inline)]
    pub use super::hkt::*;
    #[doc(inline)]
    pub use super::store::*;
}
