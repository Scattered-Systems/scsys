/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
/// The [`LinearDirection`] implementation enumerates the three pssoible movements in
/// one-dimensional space, namely:
///
/// - [`Forward`](LinearDirection::Forward) - the positive direction
/// - [`Backward`](LinearDirection::Backward) - the negative direction
/// - [`Stay`](LinearDirection::Stay) - no movement
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
pub enum LinearDirection {
    #[default]
    #[cfg_attr(feature = "serde", serde(alias = "fwd", alias = "f", alias = "right"))]
    Forward = 1,
    #[cfg_attr(feature = "serde", serde(alias = "back", alias = "left"))]
    Backward = -1,
    Stay = 0,
}

impl LinearDirection {
    pub fn from_isize(value: isize) -> Self {
        match value % 2 {
            1 => Self::Forward,
            -1 => Self::Backward,
            0 => Self::Stay,
            _ => panic!("modulo arithmetic error; resulted in a value outside of the range"),
        }
    }
    /// a functional method for initializing a new backward variant
    pub const fn backward() -> Self {
        Self::Backward
    }
    /// a functional method for initializing a new stay variant
    pub const fn stay() -> Self {
        Self::Stay
    }
    /// a functional method for initializing a new forward variant
    pub const fn forward() -> Self {
        Self::Forward
    }
    /// invert the current direction;
    ///
    /// - [`Forward`](Direction::Forward) becomes [`Backward`](Direction::Backward)
    /// - [`Backward`](Direction::Backward) becomes [`Forward`](Direction::Forward)
    /// - [`Stay`](Direction::Stay) remains as is
    pub fn invert(self) -> Self {
        use LinearDirection::*;
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

        let dir = LinearDirection::from_str("forward").ok();
        assert_eq!(dir, Some(LinearDirection::Forward));
        let inv = dir.expect("failed to parse the direction").invert();
        assert_eq!(inv, LinearDirection::Backward);
    }
}
