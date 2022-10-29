/*
    Appellation: files <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::core::BoxResult;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileHandler {
    pub path: String,
}

impl FileHandler {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

pub trait DocumentHandler {
    fn path(&self) -> String;
}
