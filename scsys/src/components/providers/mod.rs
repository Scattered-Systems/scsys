/*
    Appellation: providers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{cache::Cache, database::Database, ethereum::Web3Provider};

mod cache;
mod database;
mod ethereum;

pub enum AccessMethod {
    Uri { name: String, uri: String },
}
