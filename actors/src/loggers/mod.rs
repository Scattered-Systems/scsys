/*
    Appellation: logging <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::logger::*;

mod logger;

pub(crate) mod specs {
    pub trait Loggable {
        fn level(&self) -> String;
    }
}
