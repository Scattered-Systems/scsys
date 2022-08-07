/*
    Appellation: file <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use std::io::Read;

pub trait FileInterface: Clone + std::fmt::Debug + std::hash::Hash + PartialEq {
    fn buffer(&self) -> String {
        String::new()
    }
    fn open_file(&self) -> std::fs::File {
        match std::fs::File::open(self.filepath()) {
            Ok(file) => file,
            Err(e) => panic!("File Error: File Not Found \n{}", e),
        }
    }
    fn file_lines(&self) -> Vec<String> {
        let mut buffer = self.buffer();
        self.open_file()
            .read_to_string(&mut buffer)
            .expect("File Error");
        buffer.split("\n").map(|s: &str| s.to_string()).collect()
    }
    fn filepath(&self) -> Box<std::path::Path>;
}

/// Extract the contents of a file
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileExtractor {
    pub filepath: String,
    pub data: Vec<String>,
}

impl FileExtractor {
    fn constructor(filepath: String, data: Vec<String>) -> Self {
        Self { filepath, data }
    }
    pub fn new(path: String) -> Self {
        Self::constructor(path, Vec::new())
    }
    pub fn from<T: std::string::ToString>(path: T) -> Self {
        Self::new(path.to_string())
    }
}

impl FileInterface for FileExtractor {
    fn filepath(&self) -> Box<std::path::Path> {
        Box::from(std::path::Path::new(self.filepath.as_str()))
    }
}
