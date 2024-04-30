/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::{
    AsRefStr, Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames,
};

/// The mode of a CRUD operation.
///
#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumDiscriminants,
    EnumIs,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[cfg_attr(
    feature = "serde",
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase", untagged)
    )
)]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    name(CRUD),
    derive(
        AsRefStr,
        Display,
        EnumCount,
        EnumIs,
        EnumIter,
        EnumString,
        Hash,
        Ord,
        PartialOrd,
        VariantNames
    ),
    strum(serialize_all = "lowercase")
)]
pub enum CRUDEvent<T> {
    Create(T),
    Read(T),
    Update(T),
    Delete(T),
}

macro_rules! crud_constructors {
    ($($variant:ident.$call:ident),* $(,)?) => {
        impl<T> CRUDEvent<T> {
            $(
                pub fn $call(data: T) -> Self {
                    Self::$variant(data)
                }
            )*
        }

        impl CRUD {
            $(
                pub fn $call() -> Self {
                    Self::$variant
                }
            )*
        }
    };

}

crud_constructors!(Create.create, Read.read, Update.update, Delete.delete,);
