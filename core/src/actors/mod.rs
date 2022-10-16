/*
    Appellation: actors <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
pub use self::{actor::ActorSpec, apps::*};

pub(crate) mod actor;
pub(crate) mod apps;
pub mod extract;
pub mod generate;
pub mod handlers;
pub mod parse;
