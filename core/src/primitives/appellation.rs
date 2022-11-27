/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        An appellation is a more concise definition for the abstract naming schemes native to blockchain technologies,
        wrapping mission critical tags into a streamlined naming convention
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Appellation<T = String> {
    pub id: T,
    pub key: T, // Key is meant for use with items like a CID from IPFS
    pub label: T,
}

impl<T> std::fmt::Display for Appellation<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
