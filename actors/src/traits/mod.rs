/*
    Appellation: traits <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/
pub use self::channel::*;

pub mod channel;

pub(crate) mod prelude {
    pub use super::channel::*;
}
