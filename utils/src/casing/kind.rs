/*
    Appellation: convention <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum CaseType {
    CamelCase,
    KebabCase,
    PascalCase,
    #[default]
    SnakeCase,
}

impl CaseType {
    #[cfg(feature = "alloc")]
    /// Converts a string to the specified case.
    pub fn convert(&self, s: &str) -> alloc::string::String {
        use super::utils;
        match self {
            Self::CamelCase => utils::to_camelcase(s),
            Self::KebabCase => utils::to_kebabcase(s),
            Self::PascalCase => utils::to_pascalcase(s),
            Self::SnakeCase => utils::to_snakecase(s),
        }
    }
}
