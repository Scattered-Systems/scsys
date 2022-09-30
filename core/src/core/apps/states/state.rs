/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::Timestamp;

/// Implement the standard structure of a state
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State<S> {
    pub message: String,
    pub state: S,
    pub timestamp: Timestamp,
}

impl<S> State<S> {
    fn constructor(message: String, state: S, timestamp: Timestamp) -> Self {
        Self {
            message,
            state,
            timestamp,
        }
    }
    pub fn new(message: String, state: S) -> Self {
        Self::constructor(message, state, Timestamp::default())
    }
}

impl<S: Default> Default for State<S> {
    fn default() -> Self {
        Self::new(String::new(), S::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let actual = State::new("message".to_string(), "test");
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
