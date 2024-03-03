/*
    Appellation: messages <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::message::*;

mod message;

pub trait MessageSpec {
    fn message(&self) -> &Self;
    fn timestamp(&self) -> i64;
}
