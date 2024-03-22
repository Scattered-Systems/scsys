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
        let err = Error::new(ErrorKind::custom("custom"), msg.to_string());
        assert_eq!(err.kind(), &ErrorKind::custom("custom"));
        assert_eq!(
            Error::from(anyhow::anyhow!("")).kind(),
            &ErrorKind::unknown()
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_error_serde() {
        let err = Error::new(ErrorKind::custom("custom"), "test".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let err2: Error = serde_json::from_str(&json).unwrap();
        assert_eq!(err, err2);
    }
}
