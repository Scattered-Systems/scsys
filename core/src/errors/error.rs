/*
   Appellation: error <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::*;
use crate::id::ids::AtomicId;

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Error {
    id: AtomicId,
    kind: ErrorKind,
    message: String,
    ts: u128,
}

impl Error {
    pub fn new(kind: impl Into<ErrorKind>, message: impl ToString) -> Self {
        Self {
            id: AtomicId::new(),
            kind: kind.into(),
            message: message.to_string(),
            ts: crate::time::systime(),
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

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "*** Error ***\nKind: {}\nTimestamp: {}\nMessage:\n{}\n*** ***",
            self.kind(),
            self.timestamp(),
            self.message()
        )
    }
}

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

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind, String::new())
    }
}

macro_rules! impl_error_from {
    ($variant:ident: $($from:ty),*) => {
        $(
            impl_error_from!(@impl $variant: $from);
        )*
    };
    ($variant:ident<$n:path>.($($from:ty),*)) => {
        $(impl_error_from!(@impl $variant<$n>: $from);)*
    };
    ($variant:ident($($n:ident)::*): $($from:ty),*) => {
        $(
            impl_error_from!(@impl $variant($($n)::*): $from);
        )*
    };
    (@impl $variant:ident($($n:ident)::*): $from:ty) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::new(ErrorKind::$variant($($n)::*(err.into())), err.to_string())
            }
        }
    };
    (@impl $variant:ident<$n:path>: $from:ty) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::new(ErrorKind::$variant($n), err.to_string())
            }
        }
    };
    (@impl $variant:ident: $from:ty) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::new(ErrorKind::$variant, err.to_string())
            }
        }
    };

}

impl_error_from!(Error<ExternalError::Unknown>.(String, &str));
impl_error_from!(Parse: core::num::ParseFloatError, core::num::ParseIntError, core::str::Utf8Error);

#[cfg(feature = "std")]
impl_error_from!(Error<ExternalError::Unknown>.(Box<dyn std::error::Error>));
#[cfg(feature = "std")]
impl_error_from!(IO: std::io::Error);
#[cfg(feature = "std")]
impl_error_from!(Parse: std::string::FromUtf8Error, std::string::FromUtf16Error);
