/*
    Appellation: justifiable <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

pub trait Justifiable<I: ToString = String>: Clone + std::fmt::Display + Serialize {
    
    fn invocations(&self) -> &Vec<I>;
    fn justification(&self) -> &Justification;
}

pub struct Invocation<T: ToString = String>(T);

pub struct Originator<T: ToString>(HashSet<T>);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Justification {
    pub invocations: Vec<String>,
    pub methods: Vec<String>,
    pub origins: Vec<String>
}

impl Justification {
    pub fn new(invocations: Vec<String>, methods: Vec<String>, origins: Vec<String>) -> Self {
        Self { invocations, methods, origins }
    }
}

impl std::fmt::Display for Justification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl Justifiable for Justification {
    fn invocations(&self) -> &Vec<String> {
        &self.invocations
    }
    fn justification(&self) -> &Justification {
        self
    }

    
}

impl std::convert::From<&Justification> for Justification {
    fn from(data: &Justification) -> Self {
        data.clone()
    }
}