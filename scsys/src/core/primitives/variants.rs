/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{
    prelude::strum::{EnumString, EnumVariantNames},
    BsonOid,
};
use std::string::ToString;

#[derive(
    Clone,
    Debug,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    EnumString,
    EnumVariantNames,
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
