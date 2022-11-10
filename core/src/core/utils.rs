/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{ConfigFile, ConfigFileVec};
use glob::glob;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
    string::ToString,
};

// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
    glob(pattern)
        .expect("")
        .map(|p| ConfigFile::from(p.expect("Failed to read the pathbuf")).required(required))
        .collect::<Vec<_>>()
}

/// This function converts the file found at path (fp) into a Vec<String>
pub fn file_to_vec(fp: String) -> io::Result<Vec<String>> {
    let file_in = File::open(fp)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
/// Simple function wrapper evaluating the claim that the given information is of type f64
pub fn is_float<T: ToString>(data: &T) -> bool {
    f64::from_str(&data.to_string()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_vec() {
        let fp = "README.md".to_string();
        let a = file_to_vec(fp);
        assert!(a.is_ok());
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
