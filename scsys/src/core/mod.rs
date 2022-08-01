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
    /// Gather configuration files following the specified pattern and collect them into a vector
    pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileVec {
        glob::glob(pattern)
            .unwrap()
            .map(|path| config::File::from(path.unwrap()).required(required))
            .collect::<Vec<_>>()
    }
}
