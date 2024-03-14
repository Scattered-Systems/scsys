/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Configurable, Contextual};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Context<Cnf> {
    pub cnf: Cnf,
}

impl<Cnf> Configurable<Cnf> for Context<Cnf> {
    fn config(&self) -> &Cnf {
        &self.cnf
    }

    fn config_mut(&mut self) -> &mut Cnf {
        &mut self.cnf
    }
}

impl<Cnf> Contextual<Cnf, Self> for Context<Cnf> {
    fn context(&self) -> &Self {
        self
    }

    fn context_mut(&mut self) -> &mut Self {
        self
    }
}
