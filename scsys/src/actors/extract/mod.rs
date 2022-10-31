/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{extractor::*, files::*};

mod extractor;
mod files;

pub trait ExtractorSpec<T> {
    fn extract(&self) -> Vec<T>;
}

pub trait FileExtSpec: ExtractorSpec<String> {
    fn path(&self) -> std::path::Path;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_extractor() {
        let fp = "../README.md";
        let mut a = FileExtractor::new(fp.to_string());
        let mut b = a.clone();

        assert_eq!(a.extract(), b.extract())
    }

    #[test]
    fn test_extractor_comma() {
        let a = Extractor::new(',', "[0, 0, 0, 0]".to_string());

        assert_eq!(a.extract::<u8>(), vec![0, 0, 0, 0])
    }

    #[test]
    fn test_extractor_period() {
        let a = Extractor::new('.', "0.0.0.0".to_string());

        assert_eq!(a.extract::<u8>(), vec![0, 0, 0, 0])
    }
}
