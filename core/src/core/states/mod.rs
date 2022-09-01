/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{crud::*, power::*, state::*};

mod crud;
mod power;

mod state {
    use crate::Timestamp;

    pub trait Stateful<Cnt>: Clone + PartialEq + std::fmt::Debug + std::hash::Hash {
        fn active(&self) -> bool;
        fn context(&self, state: String) -> Cnt;
        fn message(&self, message: String) -> String {
            format!("State (message:{:?}\n)", message)
        }
        fn timestamp(&self) -> Timestamp {
            Timestamp::default()
        }
    }

    /// Implement the standard structure of a state
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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
