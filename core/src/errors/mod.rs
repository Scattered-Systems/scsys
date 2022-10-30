/*
    Appellation: errors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
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
        let expected = Error::try_from("default").ok().unwrap();
        assert_eq!(actual, expected)
    }
}
