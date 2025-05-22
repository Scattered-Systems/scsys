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
