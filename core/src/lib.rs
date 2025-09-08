/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-core
//!
//! Welcome the the scsys-core` crate, the foundational library for the [scsys.io](https://scsys.io)
//! ecosystem. This crate is primarily focused on establish a set of fundamental types, traits,
//! and utilities that are used throughout the ecosystem. Doing so allows for a natural
//! consistency to emerge across the ecosystem, while further streamlining the development
//! process.
//!
//! That being said, the general focus of the crate and its feature-gating make it ideally
//! suited for use outside of the ecosystem as well providing useful primitives such as:
//!
//! - [`Id`](id::Id) - a generic identifier type
//! - [`State`] and [`StateBase`]: dual approaches w.r.t. state management
//! - [`Timestamp`] (requires the `time` feature): a generic _timestamp_ type implementing
//!   [`Now`]
//!
#![allow(
    non_snake_case,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[cfg(feature = "alloc")]
extern crate alloc;

// re-import the `rand` & `rand_distr` crates if the `rand` feature is enabled
#[cfg(feature = "rand")]
pub use rand;
#[cfg(feature = "rand")]
pub use rand_distr;

#[doc(inline)]
#[cfg(feature = "time")]
pub use scsys_time as time;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod gsw;
    #[macro_use]
    pub mod seal;
    #[macro_use]
    pub mod wrapper;
    #[macro_use]
    pub mod wrapper_ops;
}

#[doc(inline)]
pub use self::{
    error::*,
    id::Id,
    state::{NState, State, StateBase, StateRepr, Stateful},
    types::prelude::*,
};

#[cfg(feature = "time")]
pub use scsys_time::{Now, RawTimestamp, Timestamp};

/// this module implements a set of traits and utilities for working with containers
pub mod cont;
/// this module implements various error-handling primitives and utilities
pub mod error;
/// this module defines the generic [`Id`] wrapper and its implementations
pub mod id;
/// this module provides a set of states for state-related workloads ([`State`] & [`NState`])
pub mod state;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    /// this module implements the [`LinearDirection`], an enumeration over all possible
    /// movements in one-dimensional space.
    pub mod direction;
    pub mod stages;

    pub(crate) mod prelude {
        #[allow(unused_imports)]
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::direction::*;
        #[doc(inline)]
        pub use super::stages::*;
    }

    pub(crate) mod aliases {
        #[cfg(feature = "alloc")]
        /// Type alias for a boxed error with send, sync, and static flags enabled
        pub type BoxError = alloc::boxed::Box<dyn core::error::Error + Send + Sync + 'static>;
        #[cfg(feature = "alloc")]
        /// Type alias for the standard result used
        pub type BoxResult<T = ()> = core::result::Result<T, BoxError>;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "time")]
    pub use scsys_time::prelude::*;

    pub use crate::cont::prelude::*;
    pub use crate::id::prelude::*;
    pub use crate::state::prelude::*;
    pub use crate::types::prelude::*;
}
