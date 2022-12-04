/*
    Appellation: direction <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction<S, T> {
    Input(S),
    Output(T),
}

impl<S: Clone, T: Clone> Direction<S, T> {
    pub fn input(data: S) -> Self {
        Self::Input(data)
    }
    pub fn output(data: T) -> Self {
        Self::Output(data)
    }
    pub fn shift(mut self, f: &dyn Fn(S) -> T) -> Self {
        self = transition(self.clone(), f);
        self
    }
}

impl<S, T> From<S> for Direction<S, T> {
    fn from(data: S) -> Self {
        Self::Input(data)
    }
}

impl<S, T> Default for Direction<S, T>
where
    S: Default,
{
    fn default() -> Self {
        Self::Input(Default::default())
    }
}

impl<S, T> std::fmt::Display for Direction<S, T>
where
    S: Serialize,
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub fn switch<S: Clone + From<T>, T: Clone + From<S>>(
    dir: &mut Direction<S, T>,
) -> Direction<S, T> {
    match dir.clone() {
        Direction::Input(d) => Direction::Output(d.into()),
        Direction::Output(d) => Direction::Input(d.into()),
    }
}

pub fn transition<S: Clone, T: Clone>(
    mut dir: Direction<S, T>,
    f: &dyn Fn(S) -> T,
) -> Direction<S, T> {
    dir = match dir.clone() {
        Direction::Input(d) => Direction::Output(f(d)),
        Direction::Output(d) => Direction::Output(d),
    };
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let f = |i: i32| format!("{}", i);
        let a = Direction::<i32, String>::default();
        let b = Direction::<i32, String>::from(0);
        let c = b.clone().shift(&f);
        assert_eq!(&a, &b);
        assert_eq!(&Direction::output("0".to_string()), &c);
    }
}
