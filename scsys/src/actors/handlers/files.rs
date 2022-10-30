/*
    Appellation: files <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
