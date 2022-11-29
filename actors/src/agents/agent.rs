/*
    Appellation: agent <agents>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Agency;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Agent<T>(T);

impl<T> Agent<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T: Clone + Default + Serialize + ToString> Agency for Agent<T> {
    fn init() -> Self {
        Self::new(Default::default())
    }
    fn agent(&self) -> String {
        self.to_string()
    }
}

impl<T: Clone> std::convert::From<&Agent<T>> for Agent<T> {
    fn from(data: &Agent<T>) -> Self {
        data.clone()
    }
}

impl<T: Serialize> std::fmt::Debug for Agent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl<T: Serialize> std::fmt::Display for Agent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_agent() {
        let a = Agent::<String>::default();
        let b = Agent::from(&a);

        assert_eq!(&a, &b)
    }
}
