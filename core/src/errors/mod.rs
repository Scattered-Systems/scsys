/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Errors
//!
//!
pub use self::{error::*, kinds::*};

pub(crate) mod error;
pub(crate) mod kinds;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let msg = "test";
        let err = Error::new(Errors::Error("custom".to_string()), msg.to_string());
        assert_eq!(err.kind(), &Errors::Error("custom".to_string()));
        assert_eq!(err.message(), msg);
    }
}
