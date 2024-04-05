/*
   Appellation: error <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::*;
use crate::id::ids::AtomicId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    id: AtomicId,
    kind: ErrorKind,
    message: String,
    ts: u128,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        let id = AtomicId::new();
        let ts = crate::time::systime();
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
        self.ts = crate::time::systime();
    }
}

unsafe impl Send for Error {}

unsafe impl Sync for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Error ({}) at {}\n{}",
            self.kind(),
            self.timestamp(),
            self.message()
        )
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind, String::new())
    }
}

macro_rules! impl_error_from {
    ($from:ty, $kind:expr) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::new(ErrorKind::from($kind), err.to_string())
            }
        }
    };
}

impl_error_from!(anyhow::Error, ExternalError::Unknown);

impl_error_from!(Box<dyn std::error::Error>, ExternalError::Unknown);

impl_error_from!(String, ExternalError::Unknown);

impl_error_from!(&str, ExternalError::Unknown);

impl_error_from!(std::io::Error, ErrorKind::IO);

impl_error_from!(std::num::ParseIntError, ErrorKind::Parse);

impl_error_from!(std::num::ParseFloatError, ErrorKind::Parse);

impl_error_from!(std::str::Utf8Error, ErrorKind::Parse);

impl_error_from!(std::string::FromUtf8Error, ErrorKind::Parse);

impl_error_from!(std::string::FromUtf16Error, ErrorKind::Parse);
