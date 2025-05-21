/*
    Appellation: scsys-crypto <library>
    Contrib: @FL03
*/
//! cryptographic primitives and utilities for the `scsys` ecosystem.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

#[allow(unused_imports)]
pub(crate) use scsys_core::gsw;

#[doc(inline)]
pub use self::{
    error::*, hash::prelude::*, traits::prelude::*, types::prelude::*, utils::prelude::*,
};

pub mod error;
pub mod hash;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod concat;
    pub mod hasher;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::concat::*;
        #[doc(inline)]
        pub use super::hasher::*;
    }
}

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }

    pub(crate) mod aliases {
        use generic_array::GenericArray;
        use typenum::{B0, B1, UInt, UTerm};
        /// a type alias for a generic hash output
        pub type GenericHashOutput =
            UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
        /// the [GenericHash] type alias defines a standard hash format for the crate
        pub type GenericHash<T = u8, Output = GenericHashOutput> = GenericArray<T, Output>;
    }
}

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

    #[doc(no_inline)]
    pub use crate::hash::prelude::*;
    #[doc(no_inline)]
    pub use crate::traits::prelude::*;
    #[doc(no_inline)]
    pub use crate::types::prelude::*;
    #[doc(no_inline)]
    pub use crate::utils::prelude::*;
}
