/*
    Appellation: handlers <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::files::{DocumentHandler, FileHandler};

pub(crate) mod files;

pub trait Handle<S: crate::Stateful> {
    fn state(&self) -> &S;
}

pub(crate) mod utils {}
