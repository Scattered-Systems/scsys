/*
    Appellation: extractor <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum ExtractorAction {
    Cut,
    Join,
    Skip,
    Split,
    Strip,
    Trim,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum ExtractorState {
    Complete,
    Parsing,
    Start,
}

///
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Extractor<'a> {
    pub breakpoint: char,
    pub data: String,
    pub exclude: &'a [char],
}

impl Extractor<'_> {
    fn constructor(breakpoint: char, data: String, exclude: &'static [char]) -> Self {
        Self {
            breakpoint,
            data,
            exclude,
        }
    }
    pub fn exclude_chars() -> &'static [char] {
        let to_skip = &[' ', ',', '[', ']', '.'];
        to_skip
    }
    pub fn extract<T>(self) -> Vec<T>
        where
            T: Clone + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let trimmed: &str = &self.data.trim_matches(self.exclude);
        trimmed
            .split(self.breakpoint)
            .map(|i| i.trim_matches(self.exclude).parse::<T>().unwrap())
            .collect()
    }
    pub fn new(breakpoint: char, data: String) -> Self {
        Self::constructor(breakpoint, data, Self::exclude_chars())
    }
}
