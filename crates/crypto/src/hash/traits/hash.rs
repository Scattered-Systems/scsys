/*
    appellation: hash <module>
    authors: @FL03
*/
use super::hasher::Hasher;

/// The [`Hash`] trait establishes a common interface for hashing types.
pub trait Hash {
    type Hasher: Hasher;
    type Output;
}

#[doc(hidden)]
// [`Hashable`] is a marker trait for types that can be hashed.
pub trait Hashable {
    private!();
}
