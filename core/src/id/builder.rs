/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ids::AtomicId;
use super::{IdKind, Identifier};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IdentityBuilder {
    kind: IdKind,
}

impl IdentityBuilder {
    pub fn new() -> Self {
        Self {
            kind: IdKind::default(),
        }
    }

    pub fn with_class(mut self, class: IdKind) -> Self {
        self.kind = class;
        self
    }

    pub fn build(self) -> Box<dyn Identifier> {
        match self.kind {
            IdKind::Atomic => Box::new(AtomicId::new()),
            IdKind::Object => Box::new(ObjectId::new()),
            _ => Box::new(ObjectId::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_builder() {
        let id = IdentityBuilder::new().with_class(IdKind::Object).build();
        println!("{:?}", id.to_string());
    }
}
