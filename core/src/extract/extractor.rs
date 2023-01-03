/*
    Appellation: extractor <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{extractor, DEFAULT_IGNORE_CHARS};
use std::str::FromStr;

/// Implements the formal interface for operating the extraction features
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Extractor<'a> {
    pub breakpoint: char,
    pub data: String,
    pub exclude: &'a [char],
}

impl<'a> Extractor<'a> {
    pub fn new(breakpoint: char, data: String, exclude: Option<&'a [char]>) -> Self {
        let exclude = exclude.unwrap_or(DEFAULT_IGNORE_CHARS);
        Self {
            breakpoint,
            data,
            exclude,
        }
    }
    pub fn extract<T: FromStr + ToString>(&self) -> Vec<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        extractor::<String, T>(self.breakpoint, &self.data, Some(self.exclude))
    }
}
