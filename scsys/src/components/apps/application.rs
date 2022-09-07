/*
    Appellation: application <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::Id;
use super::ApplicationMode;


#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Application {
    pub homepage: Option<String>,
    pub id: Option<Id>,
    pub key: Option<String>,
    pub label: String,
    pub mode: super
    pub name: String,
    pub secret: Option<String>
}

impl Application {
    pub fn new(homepage: Option<String>, id: Option<Id>, key: Option<String>, label: String, mode: ApplicationMode, name: String, secret: Option<String>) -> Self {
        Self { homepage, id, key, label, mode, name, secret }
    }
    pub fn slug(&self) -> String {
        self.name.clone().to_lowercase()
    }
    pub fn set_homepage(&mut self, url: &str) {
        self.homepage = Some(url.to_string());
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new(None, Some(Id::default()), None, String::new(), ApplicationMode::default(), String::new(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::Application;

    #[test]
    fn test_default_application() {
        let a = Application::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}