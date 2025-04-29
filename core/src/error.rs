/*
    Appellation: error <module>
    Contrib: @FL03
*/
pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
