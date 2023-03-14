/*
    Appellation: links <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Link<T: ToString>(T);

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Uri(String);

impl Uri {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn parse(&self) -> Result<Url> {
        let url = Url::parse(self.0.as_str())?;
        Ok(url)
    }
    pub fn uri(&self) -> String {
        self.0.clone()
    }
}
