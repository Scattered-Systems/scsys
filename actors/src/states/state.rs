/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use super::{StatePack, Stateful, StatefulExt};
use crate::messages::Message;

use serde::{Deserialize, Serialize};
use serde_json::Value;


/// Implement the standard structure of a state
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State<S: StatePack, T: Default = String> {
    pub events: Vec<String>,
    pub message: Message<T>,
    pub metadata: Value,
    pub state: S,
    pub timestamp: i64,
}

impl<S: StatePack, T: Default> State<S, T> {
    pub fn new(events: Option<Vec<String>>, message: Option<Message<T>>, state: Option<S>) -> Self {
        Self {
            events: events.unwrap_or_default(),
            message: message.unwrap_or_default(),
            metadata: Default::default(),
            state: state.unwrap_or_default(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

impl<S: StatePack, T: Default> Stateful<S> for State<S, T> {
    type Data = T;

    fn message(self) -> Message<Self::Data> {
        self.message
    }

    fn timestamp(self) -> i64 {
        self.timestamp
    }

    fn state(self) -> S {
        self.state
    }
}
impl<S: StatePack, T: Default> StatefulExt<S> for State<S, T> {
    fn update_state(&mut self, msg: Option<Message<Self::Data>>, state: S) -> &Self {
        self.message = msg.unwrap_or_default();
        self.state = state;
        self.timestamp = Self::now();
        self
    }
}
impl<S: StatePack, T: Default> Default for State<S, T> {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

impl<S: StatePack, T: Default> From<T> for State<S, T> {
    fn from(data: T) -> Self {
        Self::new(None, Some(Message::from(data)), None)
    }
}

impl<S: StatePack, T: Default> std::fmt::Display for State<S, T>
where
    S: Serialize,
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
    enum States {
        #[default]
        A = 0,
        B = 1,
    }

    impl StatePack for States {}

    impl std::fmt::Display for States {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", serde_json::to_string(&self).unwrap())
        }
    }
    #[test]
    fn test_default_state() {
        let a = State::<States, serde_json::Value>::default();
        let b = a.clone();

        assert_eq!(&a, &b);
    }
}
