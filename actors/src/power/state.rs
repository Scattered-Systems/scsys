/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::{AsRefStr, Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    AsRefStr,
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

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            enum_from_primitive!(Power(0: Off, 1: On)<$t>);
        )*
    };
}

impl_from!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
