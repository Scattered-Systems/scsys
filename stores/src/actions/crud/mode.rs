/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

/// The mode of a CRUD operation.
///
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    VariantNames,
)]
#[serde(rename_all = "snake_case", untagged)]
#[strum(serialize_all = "snake_case")]
pub enum CRUD {
    #[default]
    Create,
    Read,
    Update,
    Delete,
}

impl CRUD {
    pub fn create() -> Self {
        Self::Create
    }

    pub fn read() -> Self {
        Self::Read
    }

    pub fn update() -> Self {
        Self::Update
    }

    pub fn delete() -> Self {
        Self::Delete
    }
}
