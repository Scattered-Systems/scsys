/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{
    address::*, appellation::*, constants::*, ids::*, links::*, timestamp::*, types::*,
};

pub(crate) mod address;
pub(crate) mod appellation;
pub(crate) mod constants;
pub(crate) mod ids;
pub(crate) mod links;
pub(crate) mod timestamp;
pub(crate) mod types;
