/*
    Appellation: scsys-crypto <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::{hash::*, keys::*, primitives::*, specs::*};

pub(crate) mod hash;
pub(crate) mod keys;
pub(crate) mod primitives;
pub(crate) mod utils;

pub(crate) mod specs {
    use crate::hash::{hasher, H256};

    pub trait Hashable
    where
        Self: std::fmt::Display,
    {
        fn hash(&self) -> H256;
    }

    pub trait HashableExt: Hashable {
        fn hasher(&self, deg: Option<usize>) -> H256 {
            let s: H256 = hasher(&self).into();

            let mut res: H256 = s;
            for _ in 0..deg.unwrap_or(1) {
                res = hasher(&res.clone()).into()
            }
            res
        }
    }

    pub trait ArrayLike<T, E: std::error::Error> {
        fn flush(&mut self) -> Result<(), E>;
        fn get(&self, index: usize) -> Result<Option<T>, E>;
        fn is_empty(&self) -> Result<bool, E>;
        fn len(&self) -> Result<usize, E>;
        fn position(&self, item: &T) -> Result<Option<usize>, E>;
        fn push(&mut self, item: T) -> Result<usize, E>;
    }

    // pub trait ArrayLikeExt {
    //     type Value;
    //     fn truncate(&mut self, _len: usize) -> Result<(), MerkleMountainRangeError>;
    //     fn shift(&mut self, n: usize) -> Result<(), MerkleMountainRangeError>;
    //     fn push_front(
    //         &mut self,
    //         item: Self::Value
    //     ) -> Result<(), MerkleMountainRangeError>;
    //     fn for_each<F>(&self, f: F) -> Result<(), MerkleMountainRangeError>
    //     where
    //         F: FnMut(Result<Self::Value, MerkleMountainRangeError>);
    // }
}
