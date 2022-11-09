/*
    Appellation: scsys-crypto <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[doc(inline)]
pub use self::primitives::*;

pub mod hash;
pub mod keys;
pub(crate) mod primitives;

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
