/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[non_exhaustive]
#[repr(usize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum IdKind {
    Alphanumeric,
    Atomic,
    #[default]
    Object,
    Other(String),
}

impl IdKind {
    pub fn alphanumeric() -> Self {
        Self::Alphanumeric
    }

    pub fn atomic() -> Self {
        Self::Atomic
    }

    pub fn object() -> Self {
        Self::Object
    }
}
