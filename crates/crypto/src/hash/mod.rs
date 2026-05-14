/*
    Appellation: hash <module>
    Contrib: @FL03
*/
//! this implements various hashing primitives and utilities for cryptographic operations.
//!
//! ## Features
//!
//! - [`blake3`](https://docs.rs/blake3/latest/blake3/index.html): enables the `blake3` hashing algorithm.
#[doc(inline)]
pub use self::{h160::H160, h256::H256, traits::prelude::*, types::prelude::*};

/// this modules implements the [`H160`] type, which is a 20-byte hash output.
pub mod h160;
/// this modules implements the [`H256`] type, which is a 32-byte hash output.
pub mod h256;

pub mod traits {
    //! this module defines various traits for the [`hash`](crate::hash) module.
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod hash;
    pub mod hasher;
    pub mod raw_hash;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::hash::*;
        #[doc(inline)]
        pub use super::hasher::*;
        #[doc(inline)]
        pub use super::raw_hash::*;
    }
}

pub mod types {
    //! this module defines additional types for the [`hash`](crate::hash) module.
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

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::h160::*;
    #[doc(inline)]
    pub use super::h256::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}
