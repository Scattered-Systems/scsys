/*
    Appellation: power <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use strum_macros::{EnumString, EnumVariantNames};

/// Outlines a standard collection of power related states
#[derive(Clone, Copy, Debug, Hash, EnumString, EnumVariantNames, PartialEq, serde::Deserialize, serde::Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum PowerState {
    Off,
    On,
    Transition,
}

impl Default for PowerState {
    fn default() -> Self {
        Self::Off
    }
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
