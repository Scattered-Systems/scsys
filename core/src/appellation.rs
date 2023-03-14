/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        An appellation is a more concise definition for the abstract naming schemes native to blockchain technologies,
        wrapping mission critical tags into a streamlined naming convention

        Each appellation must contain an origin, optionally proof of ownership, optionally proof of
*/
use serde::{Deserialize, Serialize};

pub trait AppellationSpec {
    type Id;
    type Key;
    type Name: ToString;

    fn id(&self) -> &Self::Id;
    fn key(&self) -> &Self::Key;
    fn name(&self) -> &Self::Name;
}

#[derive(Clone, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Appellation<T = String> {
    pub id: T,
    pub key: T, // Key is meant for use with items like a CID from IPFS
    pub label: T,
}

impl<T> Appellation<T> {
    pub fn new(id: T, key: T, label: T) -> Self {
        Self { id, key, label }
    }
}

impl<T: Serialize> std::fmt::Debug for Appellation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl<T: Serialize> std::fmt::Display for Appellation<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
