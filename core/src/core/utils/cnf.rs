/*
    Appellation: cnf <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::{ConfigFile, ConfigFileVec};
use glob::glob;

// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    glob(pattern)
        .unwrap()
        .map(|path| ConfigFile::from(path.unwrap()).required(required))
        .collect::<Vec<_>>()
}

