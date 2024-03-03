/*
    Appellation: identity <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IdKind;

pub struct Id {
    id: String,
    kind: IdKind,
    name: String,
    timestamp: i32,
}

impl Id {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            kind: IdKind::default(),
            name: String::new(),
            timestamp: 0,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_kind(mut self, kind: IdKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_timestamp(mut self, timestamp: i32) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn build(self) -> Self {
        Self {
            id: self.id,
            kind: self.kind,
            name: self.name,
            timestamp: self.timestamp,
        }
    }
}
