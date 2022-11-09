/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use super::Stateful;
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Implement the standard structure of a state
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State<T> {
    pub events: Vec<String>,
    pub message: T,
    pub metadata: serde_json::Value,
    pub timestamp: i64,
}

impl<T> State<T> {
    pub fn new(events: Vec<String>, message: T) -> Self {
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

impl<T> Stateful<T> for State<T> {
    fn message(&self) -> &T {
        &self.message
    }

    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl<T: Default> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T> std::convert::From<T> for State<T> {
    fn from(data: T) -> Self {
        Self::new(Default::default(), data)
    }
}
