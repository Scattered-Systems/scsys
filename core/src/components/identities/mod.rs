/*
    Appellation: identies <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements basic identification utilities for usage across the ecosystem
*/
pub use self::{appellation::*, ids::*, interface::*};

pub(crate) mod appellation;
pub(crate) mod ids;

pub(crate) mod interface {
    use serde_json::Value;

    pub trait Identitfiable {
        type Id;
        fn id(&self) -> &Self::Id;
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
