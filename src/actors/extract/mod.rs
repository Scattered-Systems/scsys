/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use extractor::*;
pub use utils::*;

mod extractor;

mod utils {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        let actual: Vec<u8> = Extractor::new('.', "0.0.0.0".to_string()).extract();
        let expected: Vec<u8> = vec![0, 0, 0, 0];

        assert_eq!(&actual, &expected)
    }
}
