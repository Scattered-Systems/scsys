/// A generic function wrapper extending glob::glob
#[cfg(feature = "glob")]
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
