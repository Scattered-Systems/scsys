/*
    Appellation: scsys-derive <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{BsonOid, Timestamp};
use serde::{Deserialize, Serialize};

/// Message as a trait
pub trait IMessage {
    fn stdout<T: std::string::ToString>(&self, data: T) -> String {
        format!(
            "Timestamp: {:?}\nMessage:\n{}",
            self.timestamp(),
            data.to_string()
        )
    }
    fn timestamp(&self) -> Timestamp;
}

/// Implements a simple message structure
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Message {
    pub id: BsonOid,
    pub message: String,
    pub timestamp: Timestamp,
}

impl Message {
    pub fn new(message: String) -> Self {
        Self {
            id: BsonOid::new(),
            message,
            timestamp: Timestamp::default(),
        }
    }
    pub fn from_str<T: std::string::ToString>(message: T) -> Self {
        Self::new(message.to_string())
    }
}

impl IMessage for Message {
    fn timestamp(&self) -> Timestamp {
        self.timestamp.clone()
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn test_default_message() {
        let a = Message::from_str("Test message");
        let b = Message::default();
        assert_ne!(a, b.clone());
        assert_eq!(Message::from_str("").message, b.message)
    }
}
