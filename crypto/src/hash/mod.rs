/*
    Appellation: hash <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::{hasher::*, types::prelude::*};

pub mod hasher;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod h160;
    pub mod h256;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::h160::*;
        #[doc(inline)]
        pub use super::h256::*;
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
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::RawHash;
}

pub trait RawHash {
    private!();
}

pub trait SizedHash: RawHash {
    const N: usize;
}

macro_rules! impl_raw_hash {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_hash!(@impl $t);
        )*
    };
    ($($t:ty: $n:literal),* $(,)?) => {
        $(
            impl_raw_hash!(@sized $t => $n);
        )*
    };
    (@impl $type:ty) => {
        impl RawHash for $type {
            seal!();
        }
    };
    (@sized $type:ty => $n:literal) => {
        impl RawHash for $type {
            seal!();
        }

        impl SizedHash for $type {
            const N: usize = $n;
        }
    };
}


impl_raw_hash! {
    GenericHash,
    [u8],
}

impl_raw_hash! {
    [u8; 20]: 20,
    [u8; 32]: 32,
    H160: 20,
    H256: 32,
}

#[cfg(feature = "alloc")]
impl_raw_hash! {
    alloc::vec::Vec<u8>,
}