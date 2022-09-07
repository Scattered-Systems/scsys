/*
    Appellation: apps <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::application::Application;

mod application;

use crate::prelude::strum::{EnumString, EnumVariantNames};
use std::string::ToString;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum ApplicationMode {
    Development,
    Staging,
    Production,
    Custom(String)
}

impl Default for ApplicationMode {
    fn default() -> Self {
        Self::try_from("development").expect("Failed")
    }
}

#[cfg(test)]
mod tests {
    use super::ApplicationMode;

    #[test]
    fn test_default_application_mode() {
        let a = ApplicationMode::default();
        let b = ApplicationMode::Development;
        assert_eq!(a, b)
    }
}