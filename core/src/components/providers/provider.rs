/*
    Appellation: provider <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::{
    storage::{Cache, Database},
    Web3Provider,
};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Providers {
    pub cache: Option<Cache>,
    pub database: Option<Database>,
    pub ethereum: Option<Web3Provider>,
}
