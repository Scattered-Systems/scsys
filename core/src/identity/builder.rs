/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Id;
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
pub enum Ids {
    #[default]
    Num,
    Obj,
    Str,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IdentityBuilder {
    class: Ids,
}

impl IdentityBuilder {
    pub fn new() -> Self {
        Self {
            class: Ids::default(),
        }
    }

    pub fn with_class(mut self, class: Ids) -> Self {
        self.class = class;
        self
    }

    pub fn build(self) -> Id {
        match self.class {
            _ => Id::Object(ObjectId::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_builder() {
        let id = IdentityBuilder::new().with_class(Ids::Obj).build();
        println!("{:?}", id);
    }
}
