/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::{messages::Message, stamps::Timestamp, states::Stateful};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

/// Implement the standard structure of a state
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State<T: Display> {
    pub events: Vec<String>,
    pub message: Message<T>,
    pub metadata: Value,
    pub timestamp: i64,
}

impl<T: Display> State<T> {
    pub fn new(events: Vec<String>, message: Message<T>) -> Self {
        let metadata = Value::default();
        let timestamp = Timestamp::default().into();
        Self {
            events,
            message,
            metadata,
            timestamp,
        }
    }
}

impl<T: Clone + Default + Display + Serialize> Stateful for State<T> {
    type Data = T;

    fn message(&self) -> &Message<Self::Data> {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl<T: Default + std::fmt::Display> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Display> std::convert::From<T> for State<T> {
    fn from(data: T) -> Self {
        Self::new(Default::default(), Message::from(data))
    }
}

impl<T: Display + Serialize> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap().to_lowercase()
        )
    }
}
