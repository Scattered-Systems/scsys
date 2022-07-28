/*
    Appellation: actors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;

pub mod extract;
pub mod parse;

mod utils {
    pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileVec {
        let f = |pat: &str, opt: bool| {
            glob::glob(pat)
                .unwrap()
                .map(|path| config::File::from(path.unwrap()).required(opt))
                .collect::<Vec<_>>()
        };
        f(pattern, required)
    }
}
