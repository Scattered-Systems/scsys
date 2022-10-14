/*
    Appellation: message <moduel>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::Timestamp;
use bson::oid::ObjectId;
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
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Message {
    pub id: ObjectId,
    pub message: String,
    pub timestamp: Timestamp,
}

impl Message {
    pub fn new(message: String) -> Self {
        let id = ObjectId::new();
        let timestamp = Timestamp::default();

        Self {
            id,
            message,
            timestamp,
        }
    }
}

impl<T: std::string::ToString> std::convert::From<T> for Message {
    fn from(data: T) -> Self {
        Self::new(data.to_string())
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
