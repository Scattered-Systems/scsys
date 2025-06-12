/*
    appellation: raw_hash <module>
    authors: @FL03
*/
/// [`RawHash`] defines a common interface for raw hash types.
pub trait RawHash {
    private!();
}
/// [`SizedHash`] extends [`RawHash`] to include a constant size for the hash output.
pub trait SizedHash: RawHash {
    const N: usize;

    fn size(&self) -> usize {
        Self::N
    }
}

/*
 ************* Implementations *************
*/
use crate::hash::{GenericHash, H160, H256};

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
        impl $crate::hash::traits::RawHash for $type {
            seal!();
        }
    };
    (@sized $type:ty => $n:literal) => {
        impl_raw_hash!(@impl $type);

        impl $crate::hash::traits::SizedHash for $type {
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
