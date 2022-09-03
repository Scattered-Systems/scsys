/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{configs::*, primitives::*, utils::*};

mod configs;
mod primitives;

mod utils {
    use crate::{ConfigFile, ConfigFileVec};
    use glob::glob;

    // Gather configuration files following the specified pattern and collect them into a vector
    pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
        glob(pattern)
            .unwrap()
            .map(|path| ConfigFile::from(path.unwrap()).required(required))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::collect_config_files;

    #[test]
    fn test_collect_config_files() {
        let actual = collect_config_files("**/*.config.*", false);
        let expected = actual.clone().len();
        assert_eq!(actual.len(), expected)
    }
}
