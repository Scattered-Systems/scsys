/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
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
        p as u8
    }
}

impl From<u8> for Power {
    fn from(u: u8) -> Self {
        match u % Self::COUNT as u8 {
            1 => Power::On,
            _ => Power::Off,
        }
    }
}
