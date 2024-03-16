/*
   Appellation: kinds <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[non_exhaustive]
#[strum(serialize_all = "lowercase")]
pub enum ErrorKind {
    Async,
    Connection,
    #[default]
    Error(ExternalError),
    Execution,
    IO,
    Operation(OperationalError),
    Parse,
    Process,
    Runtime,
    Syntax,
}

impl ErrorKind {
    pub fn custom(error: impl ToString) -> Self {
        Self::Error(ExternalError::custom(error))
    }

    pub fn unknown() -> Self {
        Self::Error(ExternalError::Unknown)
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalError {
    Custom(String),
    #[default]
    Unknown,
}

impl ExternalError {
    pub fn custom(error: impl ToString) -> Self {
        Self::Custom(error.to_string())
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[non_exhaustive]
#[strum(serialize_all = "lowercase")]
pub enum OperationalError {
    #[default]
    Arithmetic,
    System,
}

impl OperationalError {
    pub fn arithmetic() -> Self {
        Self::Arithmetic
    }

    pub fn system() -> Self {
        Self::System
    }
}

macro_rules! error_kind {
    ($variant:ident, $kind:ident) => {
        impl From<$kind> for ErrorKind {
            fn from(kind: $kind) -> Self {
                Self::$variant(kind)
            }
        }
    };
}

error_kind!(Error, ExternalError);
error_kind!(Operation, OperationalError);
