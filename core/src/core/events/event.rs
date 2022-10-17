/*
    Appellation: interface <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{BsonOid, Timestamp};
use serde::{Deserialize, Serialize};

pub trait EventSpec {
    fn id(&self) -> BsonOid;
    fn key(&self) -> String;
    fn classification(&self) -> String;
    fn data(&self) -> Vec<String>;
    fn event(&self) -> &Self;
}


#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Event {
    pub id: BsonOid,
    pub key: String,
    pub class: String,
    pub data: Vec<String>,
    pub timestamp: i64
}

impl Event {
    pub fn new(key: String, class: String, data: Vec<String>) -> Self {
        let id = crate::BsonOid::new();
        let timestamp = Timestamp::default().into();
        Self { id, key, class, data, timestamp }
    }
}
