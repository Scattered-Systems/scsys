/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::merkle::{combine, merkle_hash, Leaf, Payload};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Node<T: ToString> {
    pub data: Payload<T>,
    pub hash: String,
}

impl<T: ToString> Node<T> {
    pub fn new(data: Payload<T>, hash: String) -> Self {
        Self { data, hash }
    }
}

impl<T: ToString> std::convert::From<(Node<T>, Node<T>)> for Node<T> {
    fn from(data: (Node<T>, Node<T>)) -> Self {
        let hash = merkle_hash(combine(&data.0.hash, &data.1.hash));
        let data = Payload::Node(Box::new(data.0), Box::new(data.1));
        Self::new(data, hash)
    }
}

impl<T: Clone + ToString> std::convert::From<Leaf<T>> for Node<T> {
    fn from(data: Leaf<T>) -> Self {
        Self::new(Payload::Leaf(Leaf::from(&data)), merkle_hash(data))
    }
}

impl<T: Clone + ToString> std::convert::From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self::new(Payload::Leaf(Leaf::new(data.clone())), merkle_hash(data))
    }
}
