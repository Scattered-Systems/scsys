/*
    Appellation: power <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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
        let actual = PowerState::default();
        assert_eq!(actual, PowerState::Off);
        assert_ne!(actual, PowerState::On)
    }
}
