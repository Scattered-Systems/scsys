/*
    Appellation: extractor <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Implements an extraction tool designed to iterate through a given string, collecting
/// valid data points into a vector
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Extractor<'a> {
    pub breakpoint: char,
    pub data: String,
    pub exclude: &'a [char],
}

impl Extractor<'_> {
    pub fn new(breakpoint: char, data: String) -> Self {
        let exclude = &[' ', ',', '[', ']', '.'];

        Self {
            breakpoint,
            data,
            exclude,
        }
    }
    pub fn extract<T>(&self) -> Vec<T>
    where
        T: Clone + std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let trimmed: &str = self.data.trim_matches(self.exclude);
        trimmed
            .split(self.breakpoint)
            .map(|i| i.trim_matches(self.exclude).parse::<T>().unwrap())
            .collect()
    }
}


