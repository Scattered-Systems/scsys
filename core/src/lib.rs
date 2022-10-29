/*
    Appellation: scsys-core <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#![allow(unused_imports)]
#[doc(inline)]
pub use self::{misc::*, times::*};

pub mod contexts;
pub mod errors;
pub mod events;
pub(crate) mod misc;
pub mod states;
pub(crate) mod times;
