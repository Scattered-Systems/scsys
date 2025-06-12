/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
/// An [`EventStage`] is a generic enumeration that represents the three stages of an event
/// in a system, allowing for the association of a type `T` with each stage.
/// The stages are:
///
/// - [`Before`](EventStage::Before) - the stage before the event occurs
/// - [`During`](EventStage::During) - the stage while the event is occurring
/// - [`After`](EventStage::After) - the stage after the event has occurred
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumCount,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged),
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase"),
    )
)]
#[strum_discriminants(
    name(Stage),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantArray,
        strum::VariantNames,
    ),
    strum(serialize_all = "lowercase")
)]
pub enum EventStage<A, B, C> {
    Before(A),
    During(B),
    After(C),
}

impl<A, B, C> EventStage<A, B, C> {
    /// a functional constructor for the [`After`](EventStage::After) variant
    pub const fn after(value: C) -> Self {
        Self::After(value)
    }
    /// a functional constructor for the [`Before`](EventStage::Before) variant
    pub const fn before(value: A) -> Self {
        Self::Before(value)
    }
    /// a functional constructor for the [`During`](EventStage::During) variant
    pub const fn during(value: B) -> Self {
        Self::During(value)
    }
}

impl<A, B, C> Default for EventStage<A, B, C>
where
    A: Default,
{
    fn default() -> Self {
        Self::Before(Default::default())
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::Before
    }
}

impl Stage {
    /// converts the given `isize` value into a [`Stage`] variant;
    pub const fn from_isize(value: isize) -> Self {
        use strum::EnumCount;

        match value % Self::COUNT as isize {
            1 => Self::After,
            -1 => Self::Before,
            0 => Self::During,
            _ => panic!("modulo arithmetic error; resulted in a value outside of the range"),
        }
    }
    /// a functional method for initializing a new backward variant
    pub const fn after() -> Self {
        Self::After
    }
    /// a functional method for initializing a new stay variant
    pub const fn before() -> Self {
        Self::Before
    }
    /// a functional method for initializing a new forward variant
    pub const fn during() -> Self {
        Self::During
    }
    /// invert the current direction;
    ///
    /// - [`After`](Stage::After) becomes [`Before`](Stage::Before)
    /// - [`Before`](Stage::Before) becomes [`After`](Stage::After)
    /// - [`During`](Stage::During) remains as is
    #[inline]
    pub fn invert(self) -> Self {
        use Stage::*;
        match self {
            After => Before,
            Before => After,
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        use core::str::FromStr;

        let dir = Stage::from_str("after").ok();
        assert_eq!(dir, Some(Stage::after()));
        let inv = dir.expect("failed to parse the direction").invert();
        assert_eq!(inv, Stage::before());
    }
}
