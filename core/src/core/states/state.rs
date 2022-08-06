/*
    Appellation: state <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{Deserialize, Serialize};

pub trait Stateful<Cnt>: Clone + PartialEq + std::fmt::Debug + std::hash::Hash {
    fn active(&self) -> bool;
    fn context(&self, state: String) -> Cnt;
    fn message(&self, message: String) -> String {
        message
    }
    fn timestamp(&self) -> crate::Timestamp {
        crate::Timestamp::new()
    }
}

/// Implement the standard structure of a state
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct State<Dt> {
    pub data: Vec<Dt>,
    pub timestamp: crate::Timestamp,
}

impl<Dt> State<Dt> {
    fn constructor(data: Vec<Dt>, timestamp: crate::Timestamp) -> Self {
        Self { data, timestamp }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
