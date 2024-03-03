/*
   Appellation: error <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Error {
    kind: Errors,
    message: String,
    ts: u128,
}

impl Error {
    pub fn new(kind: Errors, message: String) -> Self {
        let ts = crate::time::system_timestamp();
        Self { kind, message, ts }
    }

    pub fn kind(&self) -> &Errors {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn ts(&self) -> u128 {
        self.ts
    }

    pub fn set_kind(&mut self, kind: Errors) {
        self.kind = kind;
        self.on_update();
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.on_update();
    }

    pub fn with_kind(mut self, kind: Errors) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    fn on_update(&mut self) {
        self.ts = crate::time::system_timestamp();
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message())
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(Errors::IO, err.to_string())
    }
}
