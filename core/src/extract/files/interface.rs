/*
    Appellation: file <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use std::io::Read;

/// A complete interface for interacting with files and file-like objects
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
            Err(e) => panic!("File Error: File Not Found \n{e}"),
        }
    }
}
