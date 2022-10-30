/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{BoxResult, ConfigFile, ConfigFileVec};

use glob::glob;

// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    glob(pattern)
        .expect("")
        .map(|p| ConfigFile::from(p.expect("Failed to read the pathbuf")).required(required))
        .collect::<Vec<_>>()
}
