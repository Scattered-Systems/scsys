/*
    Appellation: storage <providers>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{cache::Cache, database::*, s3::*};

pub(crate) mod cache;
pub(crate) mod database;
pub(crate) mod s3;

pub trait UriSpec {
    fn uri(&self) -> String;
}
pub trait StorageProviderSpec {
    fn name(&self) -> String;
    fn slug(&self) -> String;
    fn uri(&self) -> String;
}
