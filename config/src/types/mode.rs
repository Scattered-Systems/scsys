/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Mode] enumerates the possible runtime modes of the application.
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
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[cfg_attr(feature = "clap", clap(name = "debug"))]
    #[cfg_attr(feature = "serde", serde(alias = "dev", alias = "development"))]
    Debug,
    #[cfg_attr(feature = "clap", clap(name = "release"))]
    #[cfg_attr(feature = "serde", serde(alias = "prod", alias = "production"))]
    Release,
}

impl Mode {
    // pub fn from_env() -> Self {
    //     std::env::var("APP_MODE")
    //         .map(|m| Self::from_str(&m))
    //         .flatten()
    //         .unwrap_or_default()
    // }

    pub fn debug() -> Self {
        Self::Debug
    }

    pub fn release() -> Self {
        Self::Release
    }
}
