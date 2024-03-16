/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{IOResult, DEFAULT_IGNORE_CHARS};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// A generic function wrapper extending glob::glob
pub fn collect_files_as<T>(f: &dyn Fn(PathBuf) -> T, pat: &str) -> Vec<T> {
    let mut files = Vec::<T>::new();
    for path in glob::glob(pat).expect("Failed to read glob pattern...") {
        if let Ok(r) = path {
            files.push(f(r))
        }
        continue;
    }
    files
}

/// A simple extraction utility for getting values from a string and converting them into a [Vec]
pub fn extractor<S, T>(bp: char, data: &S, exclude: Option<&[char]>) -> Vec<T>
where
    S: ToString,
    T: FromStr + ToString,
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
pub fn fnl_remove(data: impl ToString) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
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

/// This function attempts to convert the given input into a [std::net::SocketAddr]
pub fn try_str_to_socketaddr(
    addr: impl ToString,
) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
    addr.to_string().parse()
}
