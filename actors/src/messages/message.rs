/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::{crypto::Hashable, Hash};
use scsys_core::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Message<T = Value> {
    pub data: Option<T>,
    pub ts: i64,
}

impl<T> Message<T> {
    pub fn new(data: Option<T>) -> Self {
        let ts = Timestamp::default().into();
        Self { data, ts }
    }
    pub fn content(&self) -> &Option<T> {
        &self.data
    }
}

impl<T> Display for Message<T> where T: Serialize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl<T> From<Option<T>> for Message<T> {
    fn from(data: Option<T>) -> Self {
        Self::new(data)
    }
}

impl<T> From<T> for Message<T> {
    fn from(data: T) -> Self {
        Self::new(Some(data))
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
