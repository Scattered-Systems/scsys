/*
    Appellation: catalyst <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Converter, Transformable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Catalyst<S> {
    data: Vec<S>
}

impl<S, T> Converter<S, T> for Catalyst<S> where S: Transformable<T> {
    fn catalyst(&self, data: &S) -> T {
        todo!()
    }
}
