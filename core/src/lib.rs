/*
    Appellation: scsys-core <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use crate::{actors::*, components::*, core::*, data::*};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

pub mod prelude {
    pub use super::actors::{extract::*, parse::*};
    pub use super::components::appellations::*;
    pub use super::core::{primitives::*, utils::*};
    pub use super::data::{handlers::*, models::*};
}
