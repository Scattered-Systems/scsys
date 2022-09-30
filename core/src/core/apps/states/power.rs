/*
    Appellation: power <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

/// Implements a collection of power-related states
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum PowerState {
    #[default]
    Off,
    On,
    Transition,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_power_state() {
        let a = PowerState::default();
        let b = PowerState::On;
        assert_eq!(a, PowerState::try_from("off").expect(""));
        assert_eq!(b, PowerState::try_from("on").expect(""))
    }
}
