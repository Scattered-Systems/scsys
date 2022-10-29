/*
    Appellation: account <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Account {
    pub id: ObjectId,
    pub key: String,
    pub label: Option<String>,
    pub accounts: Option<Vec<Self>>,
    pub homepage: Option<String>,
    pub tokens: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
}
