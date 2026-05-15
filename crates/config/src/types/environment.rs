/*
    Appellation: environment <module>
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
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "d", alias = "dev"))]
    Development,
    #[cfg_attr(feature = "serde", serde(alias = "s", alias = "stag"))]
    Staging,
    #[cfg_attr(feature = "serde", serde(alias = "p", alias = "prod"))]
    Production,
}

impl Environment {
    /// returns a new instance of [`Development`](Environment::Development) variant
    pub const fn development() -> Self {
        Self::Development
    }
    /// returns a new instance of [`Staging`](Environment::Staging) variant
    pub const fn staging() -> Self {
        Self::Staging
    }
    /// returns a new instance of [`Production`](Environment::Production) variant
    pub const fn production() -> Self {
        Self::Production
    }
    #[cfg(feature = "std")]
    pub fn from_env() -> Self {
        Self::from_env_with_varname("APP_MODE")
    }
    #[cfg(feature = "std")]
    pub fn from_env_with_varname(var: &str) -> Self {
        use core::str::FromStr;
        std::env::var(var)
            .map(|m| Self::from_str(&m).ok())
            .ok()
            .flatten()
            .unwrap_or_default()
    }
}
