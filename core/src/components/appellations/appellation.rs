/*
    Appellation: appellation <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Outlines a simple name tag trait for identifying different structures throughout the ecosystem
pub trait NameTag {
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Appellation {
    pub label: String,
    pub name: String,
    pub slug: String,
}

impl Appellation {
    fn constructor(label: String, name: String, slug: String) -> Self {
        Self { label, name, slug }
    }
    pub fn new(label: String, name: String) -> Self {
        Self::constructor(label, name.clone(), Self::slug(name))
    }
    pub fn slug(name: String) -> String {
        name.to_lowercase()
    }
}

impl Default for Appellation {
    fn default() -> Self {
        Self::new(String::new(), String::new())
    }
}
