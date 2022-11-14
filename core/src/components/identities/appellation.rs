/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        An appellation is a more concise definition for the abstract naming schemes native to blockchain technologies,
        wrapping mission critical tags into a streamlined naming convention
*/
use crate::components::identities::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Appellation {
    pub id: Id,
    pub key: String, // Key is meant for use with items like a CID from IPFS
    pub label: String,
}
