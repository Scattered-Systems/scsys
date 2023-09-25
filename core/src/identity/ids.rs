/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
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
    #[default]
    Object(ObjectId),
}

impl Id {
    pub fn gen_robj() -> Self {
        Self::Object(ObjectId::new())
    }

    pub fn id_as_string(&self) -> String {
        match self {
            Self::Object(data) => data.to_hex(),
        }
    }
}

impl From<ObjectId> for Id {
    fn from(data: ObjectId) -> Self {
        Self::Object(data)
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
