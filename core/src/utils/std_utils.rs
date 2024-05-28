

#![cfg(feature = "std")]

pub use self::fs::*;

    /// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
    pub fn project_root() -> std::path::PathBuf {
        std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }

    mod fs {
        use std::fs::File;
        use std::io::{self, BufRead, BufReader};
        /// A generic function wrapper extending glob::glob
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

        /// This function converts the file found at path (fp) into a [Vec<String>]
        pub fn file_to_vec(fp: String) -> Result<Vec<String>, io::Error> {
            let file_in = File::open(fp)?;
            let file_reader = BufReader::new(file_in);
            Ok(file_reader.lines().filter_map(Result::ok).collect())
        }
    }