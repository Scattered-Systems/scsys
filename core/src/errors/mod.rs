/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Errors
//!
//!
pub use self::{error::*, kinds::*, utils::*};

pub(crate) mod error;
pub(crate) mod kinds;

pub(crate) mod utils {}

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
