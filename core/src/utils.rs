/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{BoxResult, ConfigFile, ConfigFileVec, IOResult, DEFAULT_IGNORE_CHARS};

use std::io::{BufRead, BufReader};
use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

/// A generic function wrapper extending glob::glob
pub fn collect_files_as<T>(f: &dyn Fn(PathBuf) -> T, pat: &str) -> BoxResult<Vec<T>> {
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
/// A simple extraction utility for getting values from a string and converting them into a [Vec]
pub fn extractor<S: ToString, T: FromStr + ToString>(
    bp: char,
    data: &S,
    exclude: Option<&[char]>,
) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let data = data.to_string();
    let skip = exclude.unwrap_or(DEFAULT_IGNORE_CHARS);
    let trimmed: &str = data.trim_matches(skip);
    trimmed
        .split(bp)
        .map(|i| i.trim_matches(skip).parse::<T>().unwrap())
        .collect()
}
/// This function converts the file found at path (fp) into a Vec<String>
pub fn file_to_vec(fp: String) -> IOResult<Vec<String>> {
    let file_in = File::open(fp)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(IOResult::ok).collect())
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
/// [package_name] is a simple functional wrapper for [env("CARGO_PKG_NAME")]
pub fn package_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
/// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
pub fn try_collect_config_files(pattern: &str, required: bool) -> BoxResult<ConfigFileVec> {
    let f = |p: PathBuf| ConfigFile::from(p).required(required);
    collect_files_as(&f, pattern)
}
/// This function attempts to convert the given input into a [std::net::SocketAddr]
pub fn try_str_to_socketaddr(
    addr: impl ToString,
) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
    addr.to_string().parse()
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
