/*
    appellation: hash <module>
    authors: @FL03
*/
/// A trait for hashing functionality.
pub trait Hash {
    type Hasher: Hasher;
    type Output;
}

pub trait Hashable: AsRef<[u8]> + AsMut<[u8]> {}