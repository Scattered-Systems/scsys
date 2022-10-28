/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated towards the implementation of a complete, WebAssembley native merkle tree for blockchain's and other advanced applications
*/
pub use self::{cmps::*, tree::*, utils::*};

pub(crate) mod cmps;
pub(crate) mod tree;

pub(crate) mod utils {
    use crate::hash::Hash;
    use std::string::ToString;

    pub fn merkle_hash<T: ToString>(data: T) -> String {
        Hash::from(Hash::new(data)).to_string()
    }

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
    }
}
