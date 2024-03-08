/*
   Appellation: kinds <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ErrorKind {
    Arithmetic,
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



#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
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



#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OperationalError {
    #[default]
    Arithmetic,
    System,
}

impl OperationalError {

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