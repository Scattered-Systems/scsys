/*
    Appellation: data <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use utils::*;

pub mod handlers;
pub mod models;
pub mod schemas;
pub mod structures;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Containers {
    KV(structures::KeyValue),
}


mod utils {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct FileHandler {
        pub path: String,
    }

    impl FileHandler {
        fn constructor(path: String) -> Result<Self, self::BoxError> {
            Ok(Self { path })
        }
        pub fn new(path: String) -> Self {
            match self::constructor(path) {
                Ok(v) => v,
                Err(e) => panic!("FileHandler Error: {}", e)
            }
        }
        pub fn from_path(path: Box<std::path::Path>) -> Self {
            Self::new(path.to)
        }
    }
}