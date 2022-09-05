/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{BsonOid, prelude::strum::{EnumString, EnumVariantNames}};
use std::string::ToString;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "title_case")]
pub enum Id {
    Int(i64),
    Obj(BsonOid),
    Std(String),
    Null
}
