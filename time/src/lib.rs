/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Time-related abstractions, utilities, and implementations supporting various aspects of the
//! project.
//! 
//! - [`Timestamp`] - A generic timestamp wrapper supporting various backend and representations
//!
#![allow(
    non_snake_case,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
#[cfg(feature = "std")]
pub use self::utils::*;
#[doc(inline)]
pub use self::{timestamp::Timestamp, traits::*, types::*};

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
    mod timestamp;

    mod prelude {
        #[doc(inline)]
        pub use super::now::*;
        #[doc(inline)]
        pub use super::timestamp::*;
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
