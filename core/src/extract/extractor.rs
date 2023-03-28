/*
    Appellation: extractor <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::DEFAULT_IGNORE_CHARS;
use std::str::FromStr;

/// Implements the formal interface for operating the extraction features
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Extractor {
    pub breakpoint: char,
    pub data: String,
    pub exclude: Vec<char>,
}

impl Extractor {
    pub fn new(data: String) -> Self {
        Self {
            breakpoint: char::default(),
            data,
            exclude: DEFAULT_IGNORE_CHARS.to_vec(),
        }
    }
    pub fn break_at(mut self, breakpoint: char) -> Self {
        self.breakpoint = breakpoint;
        self
    }
    pub fn exclude(mut self, exclude: impl IntoIterator<Item = char>) -> Self {
        self.exclude = Vec::from_iter(exclude);
        self
    }
    pub fn excluded(&self) -> &[char] {
        self.exclude.as_slice()
    }
    pub fn extract<T>(&self) -> Vec<T>
    where
        T: FromStr + ToString,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let exclude = self.exclude.as_slice();
        let data = self.data.clone();
        let trimmed: &str = data.trim_matches(exclude);
        trimmed
            .split(self.breakpoint)
            .map(|i| i.trim_matches(exclude).parse::<T>().unwrap())
            .collect()
    }
}
