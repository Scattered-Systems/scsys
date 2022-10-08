/*
    Appellation: files <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileHandler {
    pub path: String,
}

impl FileHandler {
    fn constructor(path: String) -> Result<Self, crate::BoxError> {
        Ok(Self { path })
    }
    pub fn new(path: String) -> Self {
        match Self::constructor(path) {
            Ok(v) => v,
            Err(e) => panic!("FileHandler Error: {}", e),
        }
    }
}

pub trait DocumentHandler {
    fn path(&self) -> String;
}
