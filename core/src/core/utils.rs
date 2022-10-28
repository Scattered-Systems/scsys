/*
    Appellation: utils <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{configure::*, files::*};
pub use crate::actors::{extract::utils::*, generate::utils::*, handlers::utils::*};
pub use crate::times::utils::*;

pub(crate) mod configure {
    use crate::{BoxResult, ConfigFile, ConfigFileVec};

    use glob::glob;

    // Gather configuration files following the specified pattern and collect them into a vector
    pub fn collect_config_files(pattern: &str, required: bool) -> ConfigFileVec {
        glob(pattern)
            .expect("")
            .map(|p| ConfigFile::from(p.expect("Failed to read the pathbuf")).required(required))
            .collect::<Vec<_>>()
    }
}

pub(crate) mod files {
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };
    /// This function converts the file found at path (fp) into a Vec<String>
    pub fn file_to_vec(fp: String) -> io::Result<Vec<String>> {
        let file_in = File::open(fp)?;
        let file_reader = BufReader::new(file_in);
        Ok(file_reader.lines().filter_map(io::Result::ok).collect())
    }
}
