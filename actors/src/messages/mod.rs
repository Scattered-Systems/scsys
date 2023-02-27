/*
    Appellation: messages <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::message::*;

pub(crate) mod message;

pub trait MessageSpec {

    fn message(&self) -> &Self;
    fn timestamp(&self) -> i64;
    
}
