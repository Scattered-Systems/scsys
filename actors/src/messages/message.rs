/*
    Appellation: message <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::{systime, BsonOid};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Message<T = Value> {
    id: String,
    data: Option<T>,
    ts: u128,
}

impl<T> Message<T> {
    pub fn new(data: Option<T>) -> Self {
        let id = BsonOid::default().to_hex();
        Self {
            id,
            data,
            ts: systime(),
        }
    }

    pub fn content(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn timestamp(&self) -> u128 {
        self.ts
    }
}

impl<T> Display for Message<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(data) = self.content() {
            return write!(
                f,
                "Timestamp: {}\n{:?}",
                self.ts,
                serde_json::to_string_pretty(data).unwrap()
            );
        }
        write!(f, "Timestamp: {}", self.ts)
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
