/*
    Appellation: scsys-derive <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::Timestamp;


pub trait IMessage {
    fn message<T: std::string::ToString>(&self, data: T) -> String {
        format!("Timestamp: {:?}\nMessage:\n{}", self.timestamp(), data.to_string())
    }
    fn timestamp(&self) -> Timestamp {
        Timestamp::default()
    }
}
