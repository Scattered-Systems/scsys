/*
    Appellation: logging <module>
    Contrib: FL03 <jo3mccain@icloud.com> (https://github.com/FL03)
*/
pub use self::logger::*;

mod logger;

pub(crate) mod specs {
    pub trait Loggable {
        fn level(&self) -> String;
    }
}
