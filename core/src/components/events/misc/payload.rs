/*
    Appellation: payload <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::stamps::Timestamp;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EventModel {
    pub id: ObjectId,
    pub created: Timestamp,
    pub dispersed: Timestamp,
    pub data: Vec<String>,
}
