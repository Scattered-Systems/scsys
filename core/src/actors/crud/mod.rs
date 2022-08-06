/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum CRUDState {
    Create,
    Read,
    Update,
    Delete,
}
