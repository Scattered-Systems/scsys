/*
    Appellation: convention <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::{
    AsRefStr, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantArray, VariantNames,
};

#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
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
    VariantArray,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "snake_case")
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum CaseType {
    CamelCase,
    KebabCase,
    PascalCase,
    SnakeCase,
}
