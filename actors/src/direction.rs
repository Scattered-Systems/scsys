/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
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
    use std::str::FromStr;

    #[test]
    fn test_direction() {
        assert_eq!(Direction::default(), Direction::Forward);
        let dir = Direction::from_str("forward").unwrap();
        assert_eq!(dir, Direction::Forward);

        assert_eq!(Direction::from_str("backward"), Ok(dir.invert()));
    }
}
