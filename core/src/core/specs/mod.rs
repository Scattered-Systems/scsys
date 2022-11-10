/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{configurable::*, contextual::*, stateful::*};

pub(crate) mod addressable;
pub(crate) mod configurable;
pub(crate) mod contextual;
pub(crate) mod eventful;
pub(crate) mod stateful;
