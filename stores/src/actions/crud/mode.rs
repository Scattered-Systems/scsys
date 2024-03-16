/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

/// The mode of a CRUD operation.
///
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
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
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[strum(serialize_all = "lowercase")]
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
