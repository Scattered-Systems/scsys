/*
    Appellation: session <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Default, Eq, Hash, PartialEq, Serialize)]
pub struct Session {
    pub events: Vec<String>,
    pub timestamp: i64,
}

impl Session {
    pub fn new() -> Self {
        let events = Vec::new();
        let timestamp = chrono::Utc::now().timestamp();
        Self { events, timestamp }
    }
    pub fn events(&self) -> &Vec<String> {
        &self.events
    }
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn update<T: ToString>(&mut self, event: T) -> &Self {
        self.events.push(event.to_string());
        self.timestamp = chrono::Utc::now().timestamp();
        self
    }
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Session::default();
        let mut b = a.clone();

        assert_eq!(&a, &b);
        assert_ne!(&a, b.update(""))
    }

    #[test]
    fn test_update() {
        let data = vec!["", "a", "c"];
        let mut a = Session::default();
        for i in data.clone() {
            a.update(i);
        }

        assert_eq!(a.events(), &data);
    }
}
