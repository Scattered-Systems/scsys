/*
    Appellation: databases <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Database {
    pub name: String,
    pub uri: String,
}

pub trait DatabaseSpec {
    fn name(&self) -> String;
    fn uri(&self) -> String;
}
