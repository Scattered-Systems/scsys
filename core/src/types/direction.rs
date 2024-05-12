/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    AsRefStr,
    Clone,
    Copy,
    Debug,
    Default,
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
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(i64)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[default]
    Forward,
    Backward,
}

impl Direction {
    pub fn backward() -> Self {
        Self::Backward
    }

    pub fn forward() -> Self {
        Self::Forward
    }

    pub fn invert(mut self) -> Self {
        self = match self {
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::default().as_ref(), "forward");
        assert_eq!(
            Direction::from_str("backward").unwrap(),
            Direction::Backward
        );
    }
}
