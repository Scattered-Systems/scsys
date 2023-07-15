/*
    Appellation: scsys-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
#[doc(inline)]
pub use self::{classify::*, identity::*, primitives::*, specs::*, utils::*};

pub mod appellation;
pub mod errors;
pub mod extract;
pub mod time;

mod classify;
mod identity;
mod primitives;
mod specs;
mod utils;

pub mod prelude {
    pub use super::appellation::*;
    pub use super::errors::*;
    pub use super::extract::*;
    pub use super::identity::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::time::*;
    pub use super::utils::*;
}
