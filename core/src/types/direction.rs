/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
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
    serde(rename_all = "lowercase", untagged)
)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[default]
    Forward = 1,
    Backward = -1,
    Stay = 0,
}

impl Direction {
    pub fn from_isize(value: isize) -> Self {
        match value % 2 {
            1 => Self::Forward,
            -1 => Self::Backward,
            0 => Self::Stay,
            _ => panic!("modulo arithmetic error; resulted in a value outside of the range"),
        }
    }
    /// a functional method for initializing a new backward variant
    pub fn backward() -> Self {
        Self::Backward
    }
    /// a functional method for initializing a new stay variant
    pub fn stay() -> Self {
        Self::Stay
    }
    /// a functional method for initializing a new forward variant
    pub fn forward() -> Self {
        Self::Forward
    }
    /// invert the current direction;
    ///
    /// - [`Forward`](Direction::Forward) becomes [`Backward`](Direction::Backward)
    /// - [`Backward`](Direction::Backward) becomes [`Forward`](Direction::Forward)
    /// - [`Stay`](Direction::Stay) remains as is
    pub fn invert(self) -> Self {
        use Direction::*;
        match self {
            Forward => Backward,
            Backward => Forward,
            Stay => Stay,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        use core::str::FromStr;

        let dir = Direction::from_str("forward").ok();
        assert_eq!(dir, Some(Direction::Forward));
        let inv = dir.expect("failed to parse the direction").invert();
        assert_eq!(inv, Direction::Backward);
    }
}
