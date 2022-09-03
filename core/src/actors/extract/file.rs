/*
    Appellation: file <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use std::io::Read;

pub trait FileInterface: Clone + std::fmt::Debug + std::hash::Hash + PartialEq {
    fn extract(&mut self) -> Vec<String> {
        let mut buffer = String::new();
        self.open_file()
            .read_to_string(&mut buffer)
            .expect("IOError");

        buffer.split("\n").map(|s: &str| s.to_string()).collect()
    }
    fn filepath(&self) -> Box<std::path::Path>;
    fn open_file(&self) -> std::fs::File {
        match std::fs::File::open(self.filepath()) {
            Ok(file) => file,
            Err(e) => panic!("File Error: File Not Found \n{}", e),
        }
    }
}

/// Extract the contents of a file
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileExtractor {
    pub filepath: String,
    pub data: Vec<String>,
}

impl FileExtractor {
    pub fn new(filepath: String) -> Self {
        Self {
            filepath,
            data: Vec::new(),
        }
    }
    pub fn from<T: std::string::ToString>(filepath: T) -> Self {
        Self::new(filepath.to_string())
    }
}

impl FileInterface for FileExtractor {
    fn filepath(&self) -> Box<std::path::Path> {
        Box::from(std::path::Path::new(self.filepath.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::{FileExtractor, FileInterface};

    #[test]
    fn test_file_extractor() {
        let fp = "../README.md";
        let mut a = FileExtractor::new(fp.to_string());
        let mut b = a.clone();

        assert_eq!(a.extract(), b.extract())
    }
}
