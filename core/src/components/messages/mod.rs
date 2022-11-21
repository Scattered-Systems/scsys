/*
    Appellation: messages <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::message::*;

pub(crate) mod message;

pub(crate) mod specs {
    use serde::Serialize;
    use std::{collections::HashSet, fmt::Display};
    
    pub trait Symbolic: Clone + Display {
        fn symbols(&self) -> &HashSet<Self>;
    }

    /// Describes the base interface for creating new messages
    pub trait MessageSpec: Clone + Display + Serialize {
        type Symbol: Symbolic;

        fn message(&self) -> &Self::Symbol;
        fn timestamp(&self) -> i64;
    }
    /// Extends the base message specification for composing more complex message pipelines
    pub trait MessageSpecExt: MessageSpec {
        type Metadata: Serialize;

        fn metadata(&self) -> &Self::Metadata;
    }
}