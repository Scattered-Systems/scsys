pub use self::utils::*;

pub trait ConfigExt {}

pub(crate) mod utils {
    use config::{FileFormat, FileSourceFile};
    use scsys::prelude::collect_files_as;

    type ConfigFile<Src = FileSourceFile, Fmt = FileFormat> = config::File<Src, Fmt>;

    /// Gather configuration files following the specified pattern and collect them into a vector
    pub fn collect_config_files(pattern: &str, required: bool) -> Vec<ConfigFile> {
        let f = |p: std::path::PathBuf| ConfigFile::from(p).required(required);
        collect_files_as(&f, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::prelude::file_to_vec;

    #[test]
    fn test_file_to_vec() {
        let fp = "../README.md".to_string();
        let a = file_to_vec(fp);
        let b = collect_config_files("**/Cargo.*", false);
        assert!(a.is_ok());
        assert!(b.len() > 0);
        assert!(!a.expect("").is_empty())
    }
}
