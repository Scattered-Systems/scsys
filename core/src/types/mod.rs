/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::direction::Direction;

pub mod direction;

pub(crate) mod prelude {
    pub use super::direction::Direction;
}
