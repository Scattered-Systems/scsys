/*
    Appellation: errors <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Error {
    Application,

    Default,
    ComputeError,
    ConnectionError,
    NodeError,
    PeerError,
    InputOutput,
    Interface,

}

impl Error {
    pub fn new(data: &str) -> Self {
        let input = data.to_string();
        Self::ComputeError
    }
    pub fn metadata() -> crate::Dictionary<Self> {
        let options = vec!["catch_all", "compute"];
        let mut res = scsys::Dictionary::new();
        res
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::Default
    }
}
