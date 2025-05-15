/*
    Appellation: std_utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
#[cfg(feature = "std")]
pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

/// A generic function wrapper extending glob::glob
#[cfg(feature = "std")]
pub fn collect_files_as<T>(f: &dyn Fn(std::path::PathBuf) -> T, pat: &str) -> Vec<T> {
    let mut files = Vec::<T>::new();
    for path in glob::glob(pat).expect("Failed to read glob pattern...") {
        if let Ok(r) = path {
            files.push(f(r))
        }
        continue;
    }
    files
}
#[cfg(feature = "std")]
/// This function converts the file found at path (fp) into a [Vec<String>]
pub fn file_to_vec(fp: String) -> Result<Vec<String>, std::io::Error> {
    use std::io::{self, BufRead};
    let file_in = std::fs::File::open(fp)?;
    let file_reader = io::BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(Result::ok).collect())
}
