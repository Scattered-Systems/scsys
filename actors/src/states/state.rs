/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::{StatePack, Stateful, StatefulExt};
use crate::messages::Message;

use chrono::Utc;
use decanter::prelude::{Hash, Hashable, H256};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<S: Clone + StatePack = States, T: Clone + Default + Serialize = H256> {
    msg: Message<T>,
    state: S,
    ts: i64,
}

impl<S: Clone + StatePack, T: Clone + Default + Serialize> State<S, T> {
    pub fn new(msg: Option<Message<T>>, state: S) -> Self {
        Self {
            msg: msg.unwrap_or_default(),
            state,
            ts: Utc::now().timestamp(),
        }
    }
}

impl<S: Clone + StatePack, T: Clone + Default + Serialize> std::fmt::Display for State<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state.to_string())
    }
}

impl<S: Clone + StatePack, T: Clone + Default + Serialize> Stateful<S> for State<S, T> {
    type Data = T;

    fn message(self) -> Message<Self::Data> {
        self.msg
    }

    fn state(self) -> S {
        self.state
    }

    fn timestamp(self) -> i64 {
        self.ts
    }
}
impl<S: Clone + StatePack, T: Clone + Default + Serialize> StatefulExt<S> for State<S, T> {
    fn update(&mut self, msg: Option<Message<Self::Data>>, state: S) {
        if let Some(m) = msg {
            self.msg = m;
        }
        self.state = state;
        self.ts = chrono::Utc::now().timestamp();
    }
}

impl<S: Clone + StatePack, T: Clone + Default + Serialize> From<&S> for State<S, T> {
    fn from(d: &S) -> Self {
        Self::new(None, d.clone())
    }
}

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
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    #[default]
    Valid = 0,
    Invalid = 1,
}

impl States {
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
}

impl StatePack for States {}

impl From<usize> for States {
    fn from(d: usize) -> Self {
        Self::from(d as i64)
    }
}

impl From<i64> for States {
    fn from(d: i64) -> Self {
        match d {
            0 => States::invalid(),
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

    #[test]
    fn test_default_state() {
        let mut a = State::<States, String>::default();
        let b = a.clone();
        assert_eq!(a.clone().state(), States::valid());

        a.update(None, States::invalid());

        assert_eq!(a.clone().state(), States::invalid());
        assert_ne!(b.timestamp(), a.timestamp())
    }
}
