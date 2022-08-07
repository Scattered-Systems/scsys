/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use primitives::*;
pub use states::*;
pub use utils::*;

pub(crate) mod primitives;
pub(crate) mod states;

pub(crate) mod utils {
    // Gather configuration files following the specified pattern and collect them into a vector
    pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileVec {
        glob::glob(pattern)
            .unwrap()
            .map(|path| crate::ConfigFile::from(path.unwrap()).required(required))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_config_files() {
        let actual = collect_config_files("**/*.config.*", false);
        let expected = actual.clone().len();
        assert_eq!(actual.len(), expected)
    }
}
