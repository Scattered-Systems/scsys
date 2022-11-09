/*
    Appellation: interface <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{events::EventModel, BsonOid, Timestamp};
use serde::{Deserialize, Serialize};

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
        let created = Timestamp::default();
        let pending_webhooks = pending_webhooks.unwrap_or(0);
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
