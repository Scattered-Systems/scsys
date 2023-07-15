/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl States {
    /// [State::Invalid] variant constructor
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn is_valid(&self) -> bool {
        *self == Self::Valid
    }
    /// [State::Valid] variant constructor
    pub fn valid() -> Self {
        Self::Valid
    }
    pub fn update(&mut self, state: Self) {
        *self = state;
    }
}



impl std::ops::Mul for States {
    type Output = States;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = self as u8 * rhs as u8;
        Self::from(res)
    }
}

impl std::ops::MulAssign for States {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<u8> for States {
    fn from(d: u8) -> Self {
        match d % 2 {
            1 => States::valid(),
            _ => States::invalid(),
        }
    }
}

impl From<usize> for States {
    fn from(d: usize) -> Self {
        Self::from(d as i64)
    }
}

impl From<i64> for States {
    fn from(d: i64) -> Self {
        match d.abs() % 2 {
            1 => States::valid(),
            _ => States::invalid(),
        }
    }
}

impl From<States> for i64 {
    fn from(d: States) -> i64 {
        d as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_states() {
        let a = States::default();
        let mut b = a;
        b *= a;
        assert_eq!(a, States::valid());
        assert_eq!(b, States::valid());
    }

    #[test]
    fn test_states_iter() {
        let a: Vec<States> = States::iter().collect();
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], States::invalid());
    }
}
