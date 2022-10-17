/*
    Appellation: interface <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{data::models::EventModel, BsonOid, Timestamp};
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
    pub created: Timestamp,

    pub account: Option<String>,
    pub classification: String,
    pub data: EventModel,
    pub pending_webhooks: i64,
}

impl Event {
    pub fn new(
        account: Option<String>,
        classification: String,
        data: EventModel,
        pending_webhooks: Option<i64>,
    ) -> Self {
        let id = crate::BsonOid::new();
        let created = Timestamp::default().into();
        let pending_webhooks = match pending_webhooks {
            Some(v) => v,
            None => 0,
        };
        Self {
            id,
            created,
            account,
            classification,
            data,
            pending_webhooks,
        }
    }
}
