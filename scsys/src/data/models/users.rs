/*
    Appellation: users <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::BsonOid;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Users {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<BsonOid>,

    pub username: String,
}

impl Users {
    pub fn new(username: String) -> Self {
        let id = Some(BsonOid::new());

        Self { id, username }
    }
}
