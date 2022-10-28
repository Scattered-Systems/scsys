/*
    Appellation: crypto <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{primitives::*, utils::*};

pub(crate) mod primitives {
    use generic_array::GenericArray;
    use typenum::{UInt, UTerm, B0, B1};

    pub type H256Hash = [u8; 32];
    pub type H160Hash = [u8; 20];

    pub type HashOutputSize = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type HashGeneric<T = u8> = GenericArray<T, HashOutputSize>;

}

pub(crate) mod utils {

}
