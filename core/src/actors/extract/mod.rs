/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{extractor::*, file::*};

pub(crate) mod extractor;
pub(crate) mod file;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        let actual: Vec<u8> = Extractor::new('.', "0.0.0.0".to_string()).extract();
        let expected: Vec<u8> = vec![0, 0, 0, 0];

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_file_extractor() {
        let path = "../README.md";
        let actual = FileExtractor::from(path);
        let expected = actual.clone();
        assert_eq!(actual.file_lines(), expected.file_lines())
    }
}
