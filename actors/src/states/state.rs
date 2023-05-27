/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::specs::StateSpec;
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
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
    #[default]
    Valid = 0,
    Invalid = 1,
}

impl States {
    pub fn all() -> Vec<Self> {
        vec![Self::Invalid, Self::Valid]
    }
    pub fn others(&self) -> Vec<Self> {
        match self {
            Self::Invalid => vec![Self::Valid],
            Self::Valid => vec![Self::Invalid],
        }
    }
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn invalidate(&mut self) {
        *self = Self::Invalid;
    }
    pub fn is_valid(&self) -> bool {
        *self == Self::Valid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
    pub fn validate(&mut self) {
        *self = Self::Valid;
    }
}

impl StateSpec for States {}

impl std::ops::Mul for States {
    type Output = States;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Invalid => match rhs {
                Self::Invalid => Self::Invalid,
                Self::Valid => Self::Valid,
            },
            Self::Valid => match rhs {
                Self::Invalid => Self::Invalid,
                Self::Valid => Self::Valid,
            },
        }
    }
}

impl std::ops::MulAssign for States {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
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
            0 => States::valid(),
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
        let a = States::all();
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], States::invalid());
    }
}
