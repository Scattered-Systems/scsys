/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::Timestamp;
use serde::{Deserialize, Serialize};

/// Implement the standard structure of a state
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<S> {
    pub message: String,
    pub state: S,
    pub timestamp: Timestamp,
}

impl<S> State<S> {
    pub fn new(message: String, state: S) -> Self {
        let timestamp = Timestamp::default();
        Self {
            message,
            state,
            timestamp,
        }
    }
}

impl<S: Default> Default for State<S> {
    fn default() -> Self {
        Self::new(String::new(), S::default())
    }
}
