/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};
use core::borrow::Borrow;

/// An `Identifier` is a type that can be used as an identifier
pub trait Identifier {
    private!();
}

/// The `Id` trait describes the behavior of a type that can be used as an id.
/// An `Id` is almost identical to an `Identifier`, but it is a trait that can be implemented for any type.
///
pub trait Id<K>
where
    K: Identifier,
{
    type Item: Borrow<K>;

    fn get(&self) -> &Self::Item;
}

#[cfg(feature = "alloc")]
pub trait IdentifierExt: Identifier
where
    Self: Copy + Eq + Ord + ToString + core::hash::Hash,
{
}

pub trait HashId: Identifier
where
    Self: Eq + core::hash::Hash,
{
}

pub trait Identify {
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
}

pub trait IdentifyMut: Identify {
    fn id_mut(&mut self) -> &mut Self::Id;
}

/*
 ************* Implementations *************
*/
impl<K, S> Id<K> for S
where
    S: Borrow<K>,
    K: Identifier,
{
    type Item = S;

    fn get(&self) -> &Self::Item {
        &self
    }
}

impl<S> Identify for S
where
    S: Identifier,
{
    type Id = S;

    fn id(&self) -> &Self::Id {
        self
    }
}

impl<Id> HashId for Id where Id: Eq + Identifier + core::hash::Hash {}

#[cfg(feature = "alloc")]
impl<Id> IdentifierExt for Id where Id: Copy + Eq + Identifier + Ord + ToString + core::hash::Hash {}

macro_rules! identifier {
    ($($t:ty),*) => {
        $(
            identifier!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl Identifier for $t {
            seal!();
        }
    };
}

identifier!(
    f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
identifier!(bool, char, &str);

#[cfg(feature = "alloc")]
identifier!(String);
