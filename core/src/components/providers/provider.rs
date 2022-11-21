/*
    Appellation: provider <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    storage::{Cache, Database},
    Web3Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Providers {
    pub cache: Option<Cache>,
    pub database: Option<Database>,
    pub ethereum: Option<Web3Provider>,
}
