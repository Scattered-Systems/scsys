/*
    Appellation: contexts <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::context::*;

pub(crate) mod context;


pub trait Configurable<Cnf> {
    fn config(&self) -> &Cnf;
    fn config_mut(&mut self) -> &mut Cnf;
}

pub trait Contextual<Cnf, Ctx>: Configurable<Cnf> {
    fn context(&self) -> &Ctx;
    fn context_mut(&mut self) -> &mut Ctx;
}
