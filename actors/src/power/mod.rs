/*
    Appellation: power <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::shutdown::*;

pub(crate) mod shutdown;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Power {
    Off,
    #[default]
    On,
}

impl Power {
    pub fn off() -> Self {
        Power::Off
    }

    pub fn on() -> Self {
        Power::On
    }

    pub fn is_off(&self) -> bool {
        matches!(self, Power::Off)
    }
}

impl From<bool> for Power {
    fn from(b: bool) -> Self {
        match b {
            true => Power::On,
            false => Power::Off,
        }
    }
}

impl From<Power> for bool {
    fn from(p: Power) -> Self {
        match p {
            Power::On => true,
            Power::Off => false,
        }
    }
}

impl From<Power> for u8 {
    fn from(p: Power) -> Self {
        match p {
            Power::On => 1,
            Power::Off => 0,
        }
    }
}

impl From<u8> for Power {
    fn from(u: u8) -> Self {
        match u % 2 {
            1 => Power::On,
            _ => Power::Off,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_power() {
        let on = {
            let tmp = Power::from_str("on");
            assert!(tmp.is_ok());
            tmp.unwrap()
        };

        assert_eq!(on, Power::On);
    }
}
