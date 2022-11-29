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
#[strum(serialize_all = "snake_case")]
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

impl std::convert::From<i64> for Id {
    fn from(data: i64) -> Self {
        Self::Int(data)
    }
}

impl std::convert::From<BsonOid> for Id {
    fn from(data: BsonOid) -> Self {
        Self::Obj(data)
    }
}

impl std::convert::From<String> for Id {
    fn from(data: String) -> Self {
        Self::Std(data)
    }
}

impl std::convert::From<&Id> for Id {
    fn from(data: &Id) -> Self {
        data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ids() {
        let actual = Id::default();
        let expected = Id::from(&actual);
        assert_eq!(actual, expected)
    }
}
