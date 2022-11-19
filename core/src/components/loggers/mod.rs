/*
    Appellation: logging <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{logger::Logger, utils::*};

mod logger;

pub trait Loggable {
    fn level(&self) -> String;
}

pub(crate) mod utils {
    
}
