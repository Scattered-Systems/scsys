/*
    Appellation: crud <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Hash, PartialEq, crate::Deserialize, crate::Serialize)]
pub enum CRUDState {
    Create,
    Read,
    Update,
    Delete,
}

///
pub trait CRUDActionSpec<T> {
    fn commit(&self, data: Vec<T>) -> Result<String, crate::BoxError>
        where
            Self: Sized;
    fn data(&self) -> Vec<T>
        where
            Self: Sized;
}
