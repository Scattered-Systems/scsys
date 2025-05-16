/*
    Appellation: mode <module>
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
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    #[default]
    #[serde(alias = "debug", alias = "dev", alias = "d")]
    Development = 0,
    #[serde(alias = "stag", alias = "staging", alias = "test", alias = "t")]
    Staging = -1,
    #[serde(alias = "prod", alias = "release", alias = "p", alias = "r")]
    Production = 1,
}

impl Mode {
    pub fn development() -> Self {
        Self::Development
    }

    pub fn production() -> Self {
        Self::Production
    }

    pub fn staging() -> Self {
        Self::Staging
    }

    pub fn as_tracing(&self) -> tracing::Level {
        use tracing::Level;
        match self {
            Self::Development => Level::DEBUG,
            Self::Staging => Level::TRACE,
            Self::Production => Level::INFO,
        }
    }
}

impl From<isize> for Mode {
    fn from(value: isize) -> Self {
        match value % 2 {
            -1 => Self::Staging,
            0 => Self::Development,
            1 => Self::Production,
            _ => unreachable!(
                "Modular arithmetic should not yield a value outside of the range: [+/-1]"
            ),
        }
    }
}

impl From<Mode> for config::Value {
    fn from(mode: Mode) -> Self {
        mode.to_string().into()
    }
}
