/*
    Appellation: extract <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{extractor::*, files::*};

mod extractor;
mod files;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractor;

    #[test]
    fn test_file_extractor() {
        let fp = "../README.md";
        let mut a = FileExtractor::new(fp.to_string());
        let mut b = a.clone();

        assert_eq!(a.extract(), b.extract())
    }

    #[test]
    fn test_extractor_comma() {
        let a = Extractor::new(',', "[0, 0, 0, 0]".to_string(), None);
        let b = extractor::<&str, u8>(',', &"[0, 0, 0, 0]", None);
        assert_eq!(a.extract::<u8>(), b)
    }

    #[test]
    fn test_extractor_period() {
        let a = Extractor::new('.', "0.0.0.0".to_string(), None);

        assert_eq!(a.extract::<u8>(), vec![0, 0, 0, 0])
    }
}
