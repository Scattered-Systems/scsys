/*
    Appellation: scsys-crypto <library>
    Contrib: @FL03
*/
//! cryptographic primitives and utilities for the `scsys` ecosystem.
#![cfg_attr(not(feature = "std"), no_std)]
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

#[allow(unused_imports)]
pub(crate) use scsys_core::gsw;

#[cfg(feature = "hash")]
pub use self::hash::prelude::*;
#[doc(inline)]
pub use self::{error::*, traits::prelude::*, utils::prelude::*};

pub mod error;
#[cfg(feature = "hash")]
pub mod hash;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod concat;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::concat::*;
    }
}

// pub mod types {
//     #[doc(inline)]
//     pub use self::prelude::*;

//     pub(crate) mod prelude {
//     }

// }

pub mod utils {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod convert;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
    }
}

pub mod prelude {
    #[doc(inline)]
    pub use crate::error::*;

    #[cfg(feature = "hash")]
    #[doc(no_inline)]
    pub use crate::hash::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}
