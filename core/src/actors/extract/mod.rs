/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use extractor::*;
pub use file::*;

pub(crate) mod extractor;
pub(crate) mod file;

pub trait IExtractor<Dt>: Clone + PartialEq + std::fmt::Debug {
    fn cache(&self) -> Vec<String>;
    fn extract(&self) -> Vec<Dt>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        let actual: Vec<u8> = Extractor::new('.', "0.0.0.0".to_string()).extract();
        let expected: Vec<u8> = vec![0, 0, 0, 0];

        assert_eq!(actual, expected)
    }
}
