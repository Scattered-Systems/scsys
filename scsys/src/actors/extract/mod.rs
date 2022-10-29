/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{extractor::*, files::*};

mod extractor;
mod files;

pub trait ExtractorSpec<T> {
    fn extract(&self) -> Vec<T>;
}

pub trait FileExtSpec: ExtractorSpec<String> {
    fn path(&self) -> std::path::Path;
}

pub(crate) mod utils {}
