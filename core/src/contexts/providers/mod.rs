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

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Provider {
    Cache(Cache),
    Database(Database),
    Web3(Web3Provider),
}
