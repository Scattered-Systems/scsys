/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use strum_macros::{EnumString, EnumVariantNames};

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
    pub fn new(data: &str) -> Self {
        match Self::info().get(data) {
            None => Self::Null,
            Some(v) => v.clone(),
        }
    }
    pub fn info() -> crate::Dictionary<Self> {
        let tmp = [
            ("create".to_string(), Self::Create),
            ("read".to_string(), Self::Read),
            ("update".to_string(), Self::Update),
            ("delete".to_string(), Self::Delete),
        ];
        crate::Dictionary::from(tmp.clone())
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
    fn test_crud_state() {
        let actual = CRUDState::new("create");
        let expected = CRUDState::try_from("create").expect("TryFrom Error");
        assert_eq!(actual, expected)
    }
}
