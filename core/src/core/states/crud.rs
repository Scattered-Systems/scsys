/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDState {
    #[default]
    Create,
    Read,
    Update,
    Delete,
}

impl CRUDState {
    pub fn new(data: &str) -> Self {
        match Self::try_from(data) {
            Ok(v) => v,
            Err(_) => panic!("{:?}", crate::Error::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CRUDState;

    #[test]
    fn test_crud_state_default() {
        let actual = CRUDState::default();
        let expected = CRUDState::Create;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_crud_state() {
        let actual = CRUDState::new("create");
        let expected = CRUDState::Create;
        assert_eq!(actual, expected)
    }
}
