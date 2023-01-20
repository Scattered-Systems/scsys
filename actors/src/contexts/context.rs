/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Configurable;
use decanter::prelude::{hasher, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context<Cnf: Configurable> {
    pub cnf: Cnf,
}

impl<Cnf: Configurable> Context<Cnf> {
    pub fn new(cnf: Cnf) -> Self {
        Self { cnf }
    }
}

impl<Cnf: Configurable> Hashable for Context<Cnf> {
    fn hash(&self) -> decanter::prelude::H256 {
        hasher(self).into()
    }
}

impl<Cnf: Configurable> std::fmt::Display for Context<Cnf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
