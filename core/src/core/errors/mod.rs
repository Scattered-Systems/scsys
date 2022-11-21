/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::error::*;

mod error;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_error_default() {
        let actual = Error::default();
        let expected = Error::Default;
        assert_eq!(actual, expected)
    }
}
