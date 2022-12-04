/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::Configurable;
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

impl<Cnf: Configurable> std::fmt::Display for Context<Cnf> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
