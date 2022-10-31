/*
    Appellation: Primitives <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use generic_array::GenericArray;
use typenum::{
    bit::{B0, B1},
    uint::{UInt, UTerm},
};

pub type H256Hash = [u8; 32];
pub type H160Hash = [u8; 20];

pub type HashOutputSize = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
pub type GenericHash<T = u8> = GenericArray<T, HashOutputSize>;
