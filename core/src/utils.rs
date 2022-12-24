/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{BoxResult, ConfigFile, ConfigFileVec};
use std::io::{self, BufRead, BufReader};
use std::{fs::File, str::FromStr, string::ToString};

/// A generic function wrapper extending glob::glob
pub fn collect_files_as<T>(f: &dyn Fn(std::path::PathBuf) -> T, pat: &str) -> BoxResult<Vec<T>> {
    let mut files = Vec::<T>::new();
    for r in glob::glob(pat)? {
        files.push(f(r?))
    }
    Ok(files)
}
/// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern).expect("Failed to find any similar files...")
}
/// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
pub fn try_collect_config_files(pattern: &str, required: bool) -> BoxResult<ConfigFileVec> {
    let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern)
}
/// This function converts the file found at path (fp) into a Vec<String>
pub fn file_to_vec(fp: String) -> io::Result<Vec<String>> {
    let file_in = File::open(fp)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
/// Remove the first and last charecters of a string
pub fn fnl_remove<T: Clone + ToString>(data: T) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
/// Simple function wrapper evaluating the claim that the given information is of type f64
pub fn is_float<T: ToString>(data: &T) -> bool {
    f64::from_str(&data.to_string()).is_ok()
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_vec() {
        let fp = "../README.md".to_string();
        let a = file_to_vec(fp);
        let b = try_collect_config_files("**/Cargo.*", false);
        assert!(a.is_ok());
        assert!(b.is_ok());
        assert!(!a.expect("").is_empty())
    }

    #[test]
    fn test_is_float() {
        let data = vec!["1", "-10", "ifjuka87"];
        assert!(is_float(&data[0]));
        assert!(is_float(&data[1]));
        assert!(!is_float(&data[2]))
    }
}
