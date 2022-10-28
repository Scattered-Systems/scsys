/*
Appellation: tree <merkle>
Contrib: FL03 <jo3mccain@icloud.com>
Description:
    Merkle Tree def...
*/
use super::{Layer, Leaf, Node};
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// Implements a complete merkle tree
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleTree<T: ToString> {
    pub leaves: Vec<Leaf<T>>,
    pub root: Node<T>,
}

impl<T: ToString> MerkleTree<T> {
    pub fn new(leaves: Vec<Leaf<T>>, root: Node<T>) -> Self {
        Self { leaves, root }
    }
    pub fn root_hash(&self) -> String {
        self.root.hash.clone()
    }
}

impl<II: IntoIterator> std::convert::From<II> for MerkleTree<II::Item>
where
    <II as IntoIterator>::Item: Clone + ToString,
{
    fn from(data: II) -> Self {
        let leaves = data.into_iter().map(Leaf::new).collect::<Vec<Leaf<_>>>();

        let mut layer: Vec<_> = leaves.iter().cloned().map(Node::from).collect();

        while layer.len() != 1 {
            layer = Layer::new(layer).into();
        }

        match layer.pop() {
            Some(root) => Self::new(leaves, root),
            None => panic!("You should not have built an empty tree"),
        }
    }
}
