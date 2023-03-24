/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Configurable, Contextual};
use decanter::prelude::{hasher, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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


