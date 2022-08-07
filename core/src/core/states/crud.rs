/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CRUDState {
    Create,
    Read,
    Update,
    Delete,
}

impl Default for CRUDState {
    fn default() -> Self {
        Self::Read
    }
}

#[cfg(test)]
mod tests {
    use super::CRUDState;

    #[test]
    fn test_crud_state() {
        let actual = CRUDState::default();
        let expected = CRUDState::Read;
        assert_eq!(actual, expected)
    }
}
