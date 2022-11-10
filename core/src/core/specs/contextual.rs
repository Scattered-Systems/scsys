/*
    Appellation: contextual <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use super::Configurable;

pub trait Contextual: ToString {
    type Cnf: Configurable;
    type Ctx;

    fn context(&self) -> &Self::Ctx;
}
