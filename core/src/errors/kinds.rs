/*
   Appellation: kinds <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(all(feature = "alloc", no_std))]
use alloc::string::String;
use smart_default::SmartDefault;
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, VariantNames};

pub trait ErrorClass {
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub enum Errors<E>
where
    E: ErrorClass,
{
    Custom(E),
    Unknown,
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    AsRefStr,
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
pub enum ErrorKind<T = String> {
    Async,
    Connection,
    #[default]
    Error(ExternalError<T>),
    Execution,
    IO,
    Operation(OperationalError),
    Parse,
    Process,
    Runtime,
    Syntax,
}

impl<T> ErrorKind<T> {
    pub fn custom(error: T) -> Self {
        Self::Error(ExternalError::custom(error))
    }

    pub fn unknown() -> Self {
        Self::Error(ExternalError::Unknown)
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    AsRefStr,
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[strum(serialize_all = "lowercase")]
pub enum ExternalError<T = String> {
    Custom(T),
    #[default]
    Unknown,
}

impl<T> ExternalError<T> {
    pub fn custom(error: T) -> Self {
        Self::Custom(error)
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    AsRefStr,
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

macro_rules! impl_kind_from {
    ($variant:ident, $kind:ident) => {
        impl From<$kind> for ErrorKind {
            fn from(kind: $kind) -> Self {
                Self::$variant(kind)
            }
        }
    };
    ($variant:ident, $($kind:ident),*) => {
        $(
            impl_kind_from!($variant, $kind);
        )*
    };
}

impl_kind_from!(Error, ExternalError);
impl_kind_from!(Operation, OperationalError);
