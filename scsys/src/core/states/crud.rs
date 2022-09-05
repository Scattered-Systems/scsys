/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::prelude::strum::{EnumString, EnumVariantNames};

#[derive(Clone, Copy, Debug, Hash, EnumString, EnumVariantNames, PartialEq, serde::Deserialize, serde::Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDState {
    Null,
    Create,
    Read,
    Update,
    Delete,
}

impl CRUDState {
    pub fn new(data: Option<&str>) -> Self {
        match data {
            None => Self::Null,
            Some(v) => {
                match Self::try_from(v) {
                    Ok(w) => w,
                    Err(_) => {
                        println!("No option labeled {}", data.unwrap());
                        Self::Null
                }
                }
            }
        }
    }
}

impl Default for CRUDState {
    fn default() -> Self {
        Self::Null
    }
}

#[cfg(test)]
mod tests {
    use super::CRUDState;

    #[test]
    fn test_crud_state_default() {
        let actual = CRUDState::default();
        let expected = CRUDState::Null;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_crud_state() {
        let actual = CRUDState::new(Some("create"));
        let expected = CRUDState::Create;
        assert_eq!(actual, expected)
    }
}
