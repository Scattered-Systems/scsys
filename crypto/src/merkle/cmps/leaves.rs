/*
    Appellation: leaves <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements the expected leaf element for composing merkle trees
*/
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Leaf<T: ToString>(T);

impl<T: ToString> Leaf<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
    pub fn data(&self) -> &T {
        &self.0
    }
}

impl<T: Clone + ToString> std::convert::From<&Leaf<T>> for Leaf<T> {
    fn from(leaf: &Leaf<T>) -> Self {
        Leaf::new(leaf.data().clone())
    }
}

impl<T: ToString> std::fmt::Display for Leaf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
