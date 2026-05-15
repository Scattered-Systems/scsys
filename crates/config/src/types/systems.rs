
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
pub enum ApplicationType {
    Desktop,
    Mobile,
    #[default]
    Web,
}

impl AppellationType {
    pub fn desktop() -> Self {
        Self::Desktop
    }

    pub fn mobile() -> Self {
        Self::Mobile
    }

    pub fn web() -> Self {
        Self::Web
    }
}