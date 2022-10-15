/*
    Appellation: file <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::io::Read;

/// Extract the contents of a file
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
}

impl std::convert::From<&str> for FileExtractor {
    fn from(path: &str) -> Self {
        Self::new(path.to_string())
    }
}

impl super::FileInterface for FileExtractor {
    fn filepath(&self) -> Box<std::path::Path> {
        Box::from(std::path::Path::new(self.filepath.as_str()))
    }
}
