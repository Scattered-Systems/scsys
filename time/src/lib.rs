/*
    Appellation: scsys-time <library>
    Created At: 2025.09.08:18:11:37
    Contrib: @FL03
*/
//! The [`time`](self) module focuses on implementing various time-related features and utilities.
//! It provides a generic [`Timestamp`] along with supporting traits and types to facilitate
//! time management.
//!
//! ## Traits
//!
//! The crate defines several key traits to support time functionalities:
//!
//! - [`RawTimestamp`] - A marker trait denoting compatible raw timestamp types
//! - [`Now`] - A trait for obtaining the current time
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

#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error!(
    "Either feature \"alloc\" or feature \"std\" must be enabled for the `time` crate to compile."
);

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
#[cfg(feature = "std")]
pub use self::utils::*;
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    timestamp::Timestamp,
    traits::*,
    types::*,
};

pub mod error;
pub mod timestamp;

pub mod types {
    //! this module contains various implementations used to support `time` related features
    #[doc(inline)]
    pub use self::prelude::*;

    mod datetime;
    mod epoch;

    mod prelude {
        #[doc(inline)]
        pub use super::datetime::*;
        #[doc(inline)]
        pub use super::epoch::*;
    }
}

pub mod traits {
    //! this moodule implements the core traits supporting the `time` module
    #[doc(inline)]
    pub use self::prelude::*;

    mod now;
    mod raw_timestamp;

    mod prelude {
        #[doc(inline)]
        pub use super::now::*;
        #[doc(inline)]
        pub use super::raw_timestamp::*;
    }
}

pub mod utils {
    //! time-related utilities
    //!
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::prelude::*;

    #[cfg(feature = "alloc")]
    mod impl_alloc;
    #[cfg(feature = "std")]
    mod impl_std;

    #[allow(unused_imports)]
    mod prelude {
        #[cfg(feature = "alloc")]
        pub use super::impl_alloc::*;
        #[cfg(feature = "std")]
        pub use super::impl_std::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::timestamp::Timestamp;
    pub use crate::traits::*;
}
