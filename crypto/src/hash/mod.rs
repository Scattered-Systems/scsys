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
        pub use super::h160::*;
        #[doc(inline)]
        pub use super::h256::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{Hashable, RawHash};
}

pub trait RawHash {}

pub trait Hash {
    type Hasher: Hasher;
    type Output;
}

pub trait Hashable: AsRef<[u8]> + AsMut<[u8]> {}
