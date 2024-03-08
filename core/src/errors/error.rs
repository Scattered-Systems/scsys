/*
   Appellation: error <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::*;
use crate::id::ids::AtomicId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Error {
    id: AtomicId,
    kind: ErrorKind,
    message: String,
    ts: u128,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        let id = AtomicId::new();
        let ts = crate::time::system_timestamp();
        Self {
            id,
            kind,
            message,
            ts,
        }
    }

    pub fn unknown(message: impl ToString) -> Self {
        Self::new(ErrorKind::unknown(), message.to_string())
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn timestamp(&self) -> u128 {
        self.ts
    }

    pub fn set_kind(&mut self, kind: ErrorKind) {
        self.kind = kind;
        self.on_update();
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
        self.on_update();
    }

    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
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

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error\nKind: {}\nTimestamp: {}\n{}", self.kind(), self.timestamp(), self.message())
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind, String::new())
    }
}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::unknown(message)
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self::unknown(message)
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::unknown(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(ErrorKind::IO, err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::new(ErrorKind::Parse, err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::new(ErrorKind::Parse, err.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::new(ErrorKind::Parse, err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::new(ErrorKind::Parse, err.to_string())
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(err: std::string::FromUtf16Error) -> Self {
        Self::new(ErrorKind::Parse, err.to_string())
    }
}






