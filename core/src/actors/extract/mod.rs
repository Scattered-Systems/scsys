/*
    Appellation: extract <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{extractor::*, file::*};

mod extractor;
mod file;

pub trait ExtractorSpec<T> {
    fn extract(&self) -> Vec<T>;
}

pub trait FileExtSpec: ExtractorSpec<String> {
    fn path(&self) -> std::path::Path;
}
