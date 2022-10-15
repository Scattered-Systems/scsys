/*
    Appellation: identies <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements basic identification utilities for usage across the ecosystem
*/
pub use self::{ids::*, interface::*};

pub(crate) mod ids;

pub(crate) mod interface {

    pub trait Identitfiable {
        fn id(&self) -> super::Id;
    }

    pub trait Appellation: Identitfiable {
        fn key(&self) -> String;
        fn label(&self) -> String;
        fn name(&self) -> String;
        fn slug(&self) -> String {
            self.name().to_lowercase()
        }
    }
}
