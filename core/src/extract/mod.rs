/*
    Appellation: extract <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{extractor::*, files::*, utils::*};

mod extractor;
mod files;

pub trait Extraction<S: ToString> {
    type Res: std::str::FromStr + ToString;

    fn extract(bp: char, data: &S, exclude: Option<&[char]>) -> Vec<Self::Res>;
}

pub(crate) mod utils {
    use crate::DEFAULT_IGNORE_CHARS;
    use std::str::FromStr;

    /// Implements the basic algorithm used by the extractor
    pub fn extractor<S: ToString, T: FromStr + ToString>(
        bp: char,
        data: &S,
        exclude: Option<&[char]>,
    ) -> Vec<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let data = data.to_string();
        let skip = exclude.unwrap_or(DEFAULT_IGNORE_CHARS);
        let trimmed: &str = data.trim_matches(skip);
        trimmed
            .split(bp)
            .map(|i| i.trim_matches(skip).parse::<T>().unwrap())
            .collect()
    }
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
