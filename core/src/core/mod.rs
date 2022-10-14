/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{apps::*, contexts::*, errors::*, events::Event, messages::*, primitives::*, utils::*};

pub(crate) mod apps;
pub(crate) mod contexts;
pub mod crypto;
pub(crate) mod errors;
pub(crate) mod events;
pub(crate) mod messages;
pub(crate) mod primitives;
pub mod states;
pub(crate) mod utils;
