/*
    Appellation: eventful <events>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::Serialize;
use std::fmt::Display;

pub trait Eventful: Clone + Default + Display + Serialize {
    fn message(&self) -> String;
    fn timestamp(&self) -> i64;
}