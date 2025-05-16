/*
    Appellation: std_error <module>
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]
use crate::error::ErrorKind;
use alloc::string::{String, ToString};

/// The [`StdError`] type is a generic error type that is generic over the error kind; this 
/// enables us to distinguish between distinctly different error types.
pub struct StdError<K: ErrorKind = String> {
    pub(crate) kind: K,
    pub(crate) message: String,
}

impl<K> StdError<K>
where
    K: ErrorKind,
{
    /// returns a reference to the kind of error
    pub fn kind(&self) -> &K {
        &self.kind
    }
    /// returns a mutable reference to the kind of error
    pub fn kind_mut(&mut self) -> &mut K {
        &mut self.kind
    }
    /// returns a reference to the message
    pub fn message(&self) -> &str {
        &self.message
    }
    /// update the kind before returning a mutable reference to the current instance.
    pub fn set_kind(&mut self, value: K) -> &mut Self {
        self.kind = value;
        self
    }
    /// update the message before returning a mutable reference to the current instance.
    pub fn set_message<V: ToString>(&mut self, value: V) -> &mut Self {
        self.message = value.to_string();
        self
    }
    /// consumes the current instance to create another with the given kind
    pub fn with_kind(self, kind: K) -> Self {
        Self { kind, ..self }
    }
    /// consumes the current instance to create another with the given message.
    pub fn with_message(self, message: String) -> Self {
        Self { message, ..self }
    }
}

unsafe impl<K: ErrorKind> Send for StdError<K> {}

unsafe impl<K: ErrorKind> Sync for StdError<K> {}
