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
#![cfg_attr(not(feature = "std"), no_std)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[cfg(feature = "alloc")]
extern crate alloc;
// re-import the `rand` & `rand_distr` crates if the `rand` feature is enabled
#[cfg(feature = "rand")]
#[doc(no_inline)]
pub use rand;
#[cfg(feature = "rand")]
#[doc(no_inline)]
pub use rand_distr;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod gsw;
    #[macro_use]
    pub mod seal;
    #[macro_use]
    pub mod wrapper;
}

#[doc(inline)]
pub use self::{
    error::*,
    id::Id,
    state::{NState, State, StateBase, StateRepr, Stateful},
    time::{Now, RawTimestamp, Timestamp},
    types::prelude::*,
};

pub mod error;
pub mod id;
pub mod state;
pub mod time;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod direction;

    pub(crate) mod prelude {
        #[allow(unused_imports)]
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::direction::*;
    }

    pub(crate) mod aliases {
        #[cfg(feature = "alloc")]
        /// Type alias for a boxed error with send, sync, and static flags enabled
        pub type BoxError = alloc::boxed::Box<dyn core::error::Error + Send + Sync + 'static>;
        #[cfg(feature = "alloc")]
        /// Type alias for the standard result used
        pub type BoxResult<T = ()> = core::result::Result<T, BoxError>;
        #[cfg(feature = "std")]
        /// Type alias wrapping a locked, thread-safe structure with a [Mutex] in an [Arc]
        pub type Arcm<T> = std::sync::Arc<std::sync::Mutex<T>>;
        #[cfg(feature = "std")]
        /// Type alias for [std::io::Result]
        pub type IOResult<T = ()> = std::io::Result<T>;
    }
}

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::error::*;
    #[doc(no_inline)]
    pub use crate::id::prelude::*;
    #[doc(no_inline)]
    pub use crate::state::prelude::*;
    #[doc(no_inline)]
    pub use crate::time::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
}
