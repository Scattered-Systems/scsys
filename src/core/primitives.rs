/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

mod utils {
    pub fn extractor<T>(string: String, breakpoint: char) -> Vec<T>
        where
            T: Clone + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let exclude: &[char] = &[' ', ',', '[', ']', '.'];
        let trimmed: &str = &string.trim_matches(exclude);
        trimmed
            .split(breakpoint)
            .map(|i| i.trim_matches(exclude).parse::<T>().unwrap())
            .collect()
    }
}
