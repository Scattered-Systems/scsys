/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct State {
    message: String,
    state: States,
}

impl State {
    pub fn new(message: impl ToString, state: States) -> Self {
        Self {
            message: message.to_string(),
            state,
        }
    }
    /// Sets the state to [States::Invalid]
    pub fn invalidate(&mut self) {
        self.state = States::Invalid;
    }
    /// Returns true if the state is [States::Valid]
    pub fn is_valid(&self) -> bool {
        self.state == States::Valid
    }
    /// Returns the message
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_message(&mut self, message: impl ToString) {
        self.message = message.to_string();
    }
    pub fn set_state(&mut self, state: States) {
        self.state = state;
    }
    /// Returns the current state
    pub fn state(&self) -> States {
        self.state
    }

    pub fn update(&mut self, state: State) {
        *self = state;
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl From<States> for State {
    fn from(state: States) -> Self {
        Self::new("", state)
    }
}

impl From<State> for States {
    fn from(q: State) -> Self {
        q.state
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
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
    Serialize,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
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
