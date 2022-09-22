/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::BsonOid;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "title_case")]
pub enum Id {
    Int(i64),
    Obj(BsonOid),
    Std(String),
    Null,
}

impl Default for Id {
    fn default() -> Self {
        Self::Obj(BsonOid::new())
    }
}
