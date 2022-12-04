/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Message<T = Value> {
    pub data: Vec<T>,
    pub timestamp: i64,
}

impl<T> Message<T> {
    pub fn new(data: Vec<T>) -> Self {
        let timestamp = Utc::now().timestamp();
        Self { data, timestamp }
    }
    pub fn content(&self) -> &Vec<T> {
        &self.data
    }
    pub fn push(&mut self, data: T) -> &Self {
        self.data.push(data);
        self
    }
}

impl<T> std::convert::From<Vec<T>> for Message<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}

impl<T> std::convert::From<T> for Message<T> {
    fn from(data: T) -> Self {
        Self::new(vec![data])
    }
}

impl<T: Serialize> Display for Message<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_message() {
        let a = Message::<String>::default();
        assert_eq!(&a.data, &a.data)
    }
}
