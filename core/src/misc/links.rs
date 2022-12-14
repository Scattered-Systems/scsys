/*
    Appellation: links <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Link<T: std::string::ToString>(T);

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Uri(pub String);

impl Uri {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn parse(&self) -> Result<Url> {
        let url = Url::parse(self.0.as_str())?;
        Ok(url)
    }
}
