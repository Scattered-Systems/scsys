/*
    Appellation: stateful <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::messages::Message;
use serde::Serialize;

pub trait Stateful: Clone + Default + Serialize + std::fmt::Display {
    type Data: std::fmt::Display;

    fn message(&self) -> &Message<Self::Data>;
    fn timestamp(&self) -> i64;
}

pub trait StatefulExt: Stateful {
    fn agency(&self) -> String;
    fn catalyst<S, T>(&mut self, f: &dyn Fn(S) -> T) -> Vec<T>;
    fn tags(&self) -> Vec<String>;
}
