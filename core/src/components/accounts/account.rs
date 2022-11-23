/*
    Appellation: account <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Account {
    pub handle: String,
    pub id: ObjectId,
    pub key: String,
    pub data: Vec<Option<AccountMetadata>>,
}

impl Account {
    pub fn new(handle: String, key: String) -> Self {
        let id = ObjectId::new();
        Self {
            handle,
            id,
            key,
            data: Default::default(),
        }
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
pub enum AccountMetadata {
    Account(Account),
    Generic(serde_json::Value),
}
impl Default for AccountMetadata {
    fn default() -> Self {
        Self::Generic(Default::default())
    }
}

impl std::fmt::Display for AccountMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
