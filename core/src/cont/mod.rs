/*
    appellation: store <module>
    authors: @FL03
*/
//! this module is focused on defining a set of traits and types for abstracting the behaviourds
//! of an entity capable of storing some data.
#[doc(inline)]
pub use self::{container::*, error::*, traits::prelude::*};

/// this module implements the [`ContainerBase`] type, which is a base type for containers that
/// use a store to manage their data.
pub mod container;
/// this module defiens the [`StoreError`] enum for handling various errors that can occur with
/// stores
pub mod error;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;
    /// this module defines the [`Entry`] trait for establishing a common interface for
    /// entries in a store, which are key-value pairs.
    pub mod entry;
    /// this module defines the [`Get`] trait for establishing a common interface for
    /// retrieving values from a store.
    pub mod get;
    /// this module defines the [`HKT`] trait for establishing a common interface
    /// for higher-kinded types, which are types that take other types as parameters.
    pub mod hkt;
    /// this module defines the [`RawContainer`] trait for establishing a core interface for
    /// various representations of a container.
    pub mod raw_container;
    /// this module defines the [`RawStore`] trait that extends the [`RawContainer`] trait to
    /// establish an interface for key-value stores.
    pub mod raw_store;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::entry::*;

        #[doc(inline)]
        pub use super::raw_container::*;
        #[doc(inline)]
        pub use super::raw_store::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::traits::prelude::*;
}
