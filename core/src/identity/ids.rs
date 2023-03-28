/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{generate_random_number, generate_random_string};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum Id {
    Int(i64),
    #[default]
    Object(ObjectId),
    String(String),
}

impl Id {
    pub fn gen_rint() -> Self {
        Self::Int(generate_random_number())
    }
    pub fn gen_robj() -> Self {
        Self::Object(ObjectId::new())
    }
    //
    pub fn gen_rstr(len: Option<usize>) -> Self {
        Self::String(generate_random_string(len.unwrap_or(12)))
    }
}

impl From<i64> for Id {
    fn from(data: i64) -> Self {
        Self::Int(data)
    }
}

impl From<ObjectId> for Id {
    fn from(data: ObjectId) -> Self {
        Self::Object(data)
    }
}

impl From<String> for Id {
    fn from(data: String) -> Self {
        Self::String(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ids() {
        let actual = Id::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
