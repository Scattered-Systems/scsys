/*
   Appellation: error <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::kinds::*;
use crate::id::AtomicId;
#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

#[derive(Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Error<K = String> {
    id: AtomicId,
    kind: Errors<K>,
    message: String,
}

impl<K> Error<K> {
    pub fn new(kind: impl Into<Errors<K>>, message: impl ToString) -> Self {
        Self {
            id: AtomicId::new(),
            kind: kind.into(),
            message: message.to_string(),
        }
    }

    pub fn unknown(message: impl ToString) -> Self {
        Self::new(Errors::<K>::unknown(), message.to_string())
    }

    pub fn id(&self) -> usize {
        *self.id
    }

    pub fn kind(&self) -> &Errors<K> {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_kind(&mut self, kind: Errors<K>) {
        self.kind = kind;
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn with_kind(mut self, kind: Errors<K>) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

unsafe impl<K> Send for Error<K> {}

unsafe impl<K> Sync for Error<K> {}

impl<K> core::fmt::Debug for Error<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "[{kind}]: {msg}",
            kind = self.kind(),
            msg = self.message()
        )
    }
}

impl<K> core::fmt::Display for Error<K> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "[{kind}]: {msg}",
            kind = self.kind(),
            msg = self.message()
        )
    }
}

impl<K> core::error::Error for Error<K> {}

#[cfg(feature = "alloc")]
impl<K> From<Errors<K>> for Error<K> {
    fn from(kind: Errors<K>) -> Self {
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
        impl<K> From<$from> for Error<K> {
            fn from(err: $from) -> Self {
                Self::new(Errors::$variant($($n)::*(err.into())), err.to_string())
            }
        }
    };
    (@impl $variant:ident<$n:path>: $from:ty) => {
        impl<K> From<$from> for Error<K> {
            fn from(err: $from) -> Self {
                Self::new(Errors::$variant($n), err.to_string())
            }
        }
    };
    (@impl $variant:ident: $from:ty) => {
        impl<K> From<$from> for Error<K> {
            fn from(err: $from) -> Self {
                Self::new(Errors::$variant, err.to_string())
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
