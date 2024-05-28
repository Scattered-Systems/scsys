/*
   Appellation: kinds <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(all(feature = "alloc", no_std))]
use alloc::string::String;
use smart_default::SmartDefault;
use strum::{AsRefStr, Display, EnumCount, EnumIs, VariantNames};

pub trait ErrorTy {
    fn name(&self) -> &str;
}


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
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[non_exhaustive]
#[strum(serialize_all = "lowercase")]
pub enum Errors<T = String> {
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

impl<T> Errors<T> {
    pub fn custom(error: T) -> Self {
        Self::Error(ExternalError::custom(error))
    }

    pub fn unknown() -> Self {
        Self::Error(ExternalError::Unknown)
    }
}






macro_rules! err {
    ($name:ident $($rest:tt)*) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::VariantNames,
        )]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = "lowercase", untagged)
        )]
        #[non_exhaustive]
        #[strum(serialize_all = "lowercase")]
        pub enum $name $($rest)*
    };
}

macro_rules! impl_kind_from {
    ($variant:ident, $kind:ident) => {
        impl From<$kind> for Errors {
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

err! {
    OperationalError {
        Arithmetic,
        System,
    }
}

err! {
    ExternalError<T = String> {
        Custom(T),
        Unknown,
    }
}

impl<T> ExternalError<T> {
    pub fn custom(error: T) -> Self {
        Self::Custom(error)
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}


impl<T> Default for ExternalError<T> {
    fn default() -> Self {
        Self::Unknown
    }
}


impl_kind_from!(Error, ExternalError);
impl_kind_from!(Operation, OperationalError);
