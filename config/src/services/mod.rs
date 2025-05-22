/*
    Appellation: services <module>
    Contrib: @FL03
*/
//! the `services` module provides pre-defined schemas for common services
#[doc(inline)]
pub use self::prelude::*;

pub mod database;
pub mod logger;
pub mod network;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::database::*;
    #[doc(inline)]
    pub use super::logger::*;
    #[doc(inline)]
    pub use super::network::*;
}
