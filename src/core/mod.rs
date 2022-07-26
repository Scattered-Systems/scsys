/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use primitives::*;
pub use utils::*;

mod primitives;

mod utils {
    pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileVec {
        let f = |pat: &str, opt: bool| {
            glob::glob(pat)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(opt))
                .collect::<Vec<_>>()
        };
        f(pattern, required)
    }

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

    pub struct Extract<Dt>
        where
            Dt: Clone + std::str::FromStr,
            <Dt as std::str::FromStr>::Err: std::fmt::Debug,
    {
        pub data: Vec<Dt>,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor() {
        let a: Vec<u8> = extractor("0.0.0.0".to_string(), '.');
        let b: Vec<u8> = extractor("[0, 0, 0, 0]".to_string(), ',');
        assert_eq!(&a, &b)
    }
}
