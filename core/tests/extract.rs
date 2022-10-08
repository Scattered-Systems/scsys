#[cfg(test)]
mod tests {
    use scsys_core::extract::{Extractor, FileExtractor, FileInterface};

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