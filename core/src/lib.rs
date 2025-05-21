/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This crate works to provide a set of utilities for working with state, time, and synchronization in Rust.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod builder;
    #[macro_use]
    pub mod fmt;
    #[macro_use]
    pub mod get;
    #[macro_use]
    pub mod gsw;
    #[macro_use]
    pub mod seal;
    #[macro_use]
    pub mod wrapper;
}

#[doc(inline)]
pub use self::{state::NState, types::prelude::*, utils::*};

#[doc(inline)]
pub use self::error::*;

pub(crate) mod utils;

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
    #[doc(no_inline)]
    pub use crate::utils::*;
}
