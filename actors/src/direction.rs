/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
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
    Serialize,
    VariantNames,
)]
#[repr(i64)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Direction {
    #[default]
    Input,
    Output,
}

impl Direction {
    pub fn input() -> Self {
        Self::Input
    }
    pub fn output() -> Self {
        Self::Output
    }
    pub fn invert(mut self) -> Self {
        self = match self {
            Self::Input => Self::Output,
            Self::Output => Self::Input,
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
        assert_eq!(Direction::default(), Direction::Input);
        let dir = Direction::from_str("input").unwrap();
        assert_eq!(dir, Direction::Input);

        assert_eq!(Direction::from_str("output"), Ok(dir.invert()));
    }
}
