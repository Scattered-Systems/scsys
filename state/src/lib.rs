/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The [`state`](self) module provides various abstractions and implementations for working
//! with states within Rust.
//!
//! - [`State<Q>`] - A generic wrapper type that encapsulates a state of type `Q`, providing
//!   methods for state manipulation and retrieval.
//! - [`StateBase<Q, K>`] - A dynamic state representation capable of generically associating
//!   some state `Q` with a particular _kind_ `K`.
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

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    nstate::StateBase,
    state::State,
    traits::prelude::*,
    types::prelude::*,
};

pub mod error;
pub(crate) mod nstate;
pub(crate) mod state;

mod impls {
    mod impl_state;
    mod impl_state_ops;

    #[allow(deprecated)]
    mod impl_deprecated;
}

pub mod traits {
    //! this module implements various traits supporting the [`State`](super::State) type
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod kind;
    pub mod state;
    pub mod state_repr;
    pub mod stateful;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::kind::*;
        #[doc(inline)]
        pub use super::state::*;
        #[doc(inline)]
        pub use super::state_repr::*;
        #[doc(inline)]
        pub use super::stateful::*;
    }
}

pub mod types {
    //! additional types for the [`state`](crate::state) module
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod kinds;
    pub mod nary;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::kinds::*;
        #[doc(inline)]
        pub use super::nary::*;
    }

    pub(crate) mod aliases {
        use super::Nary;
        use crate::nstate::StateBase;

        /// A type alias for a [`StateBase`] equipped with a [`Nary`] kind of state
        pub type NState<T, const N: usize = 4> = StateBase<T, Nary<N>>;
    }
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::nstate::StateBase;
    pub use crate::state::State;
    pub use crate::traits::prelude::*;
    pub use crate::types::prelude::*;
}
