/*
    Appellation: agent <agents>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Agency;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Agent<T>(T);

impl<T> Agent<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T> Agency for Agent<T>
where
    T: Clone + Default + Serialize + ToString,
{
    fn init() -> Self {
        Self::new(Default::default())
    }
    fn agent(&self) -> String {
        self.to_string()
    }
}

impl<T> std::fmt::Display for Agent<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
