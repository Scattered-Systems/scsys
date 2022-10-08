/*
    Appellation: file <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use std::io::Read;

pub trait FileInterface {
    fn extract(&mut self) -> Vec<String> {
        let mut buffer = String::new();
        self.open_file()
            .read_to_string(&mut buffer)
            .expect("IOError");

        buffer.split('\n').map(|s: &str| s.to_string()).collect()
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
#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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

impl FileInterface for FileExtractor {
    fn filepath(&self) -> Box<std::path::Path> {
        Box::from(std::path::Path::new(self.filepath.as_str()))
    }
}
