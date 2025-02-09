/*
    Appellation: hkt <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Higher Kinded Types
#[doc(inline)]
pub use self::applicative::*;

mod applicative;

pub mod hkt;

pub(crate) mod prelude {
    pub use super::applicative::*;
}
