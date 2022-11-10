/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context<Cnf: Configurable> {
    pub settings: Cnf,
}

impl<Cnf: Configurable> Context<Cnf> {
    pub fn new(settings: Cnf) -> Self {
        Self { settings }
    }
}
