/*
    Appellation: identies <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements basic identification utilities for usage across the ecosystem
*/
pub use self::{appellation::*, ids::*, interface::*};

pub(crate) mod appellation;
pub(crate) mod ids;

pub(crate) mod interface {
    use super::Id;
    use crate::core::Timestamp;
    use serde_json::Value;

    pub trait Identitfiable {
        fn id(&self) -> Id;
        fn gen_timestamp(&self) -> Timestamp {
            Timestamp::default()
        }
    }

    pub trait Justification {
        fn authorization(&self) -> Value;
    }

    pub trait AppellationSpec: Identitfiable {
        fn key(&self) -> String;
        fn label(&self) -> String;
    }

    pub trait InputName {
        fn name(&self) -> String;
        fn slug(&self) -> String {
            self.name().to_lowercase()
        }
    }
}
