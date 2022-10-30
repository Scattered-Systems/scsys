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

pub trait ArrayLike {
    type Value;
    type Error: std::error::Error;
    fn len(&self) -> Result<usize, Self::Error>;
    fn is_empty(&self) -> Result<bool, Self::Error>;
    fn push(&mut self, item: Self::Value) -> Result<usize, Self::Error>;
    fn get(&self, index: usize) -> Result<Option<Self::Value>, Self::Error>;
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn position(&self, item: &Self::Value) -> Result<Option<usize>, Self::Error>;
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