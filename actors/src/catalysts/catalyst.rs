/*
    Appellation: catalyst <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{messages::Message, Direction};
use serde::{Deserialize, Serialize};
use std::{convert::From, fmt::Display};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Catalyst<S: Clone, T: Clone> {
    pub data: Vec<Direction<Message<S>, Message<T>>>,
}

impl<S: Clone, T: Clone> Catalyst<S, T> {
    pub fn new() -> Self {
        let data = Vec::new();

        Self { data }
    }
}

impl<S: Clone, T: Clone> Default for Catalyst<S, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone, T: Clone> std::fmt::Display for Catalyst<S, T>
where
    S: Serialize,
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
