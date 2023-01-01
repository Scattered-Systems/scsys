/*
    Appellation: servers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub fn address(&self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(self.pieces())
    }
    pub fn pieces(&self) -> ([u8; 4], u16) {
        let host: [u8; 4] = extractor('.', &self.host.clone(), None)
            .try_into()
            .ok()
            .unwrap();
        (host, self.port)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new("0.0.0.0".to_string(), 8080)
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the server locally at http://{}:{}",
            self.host, self.port
        )
    }
}

const DEFAULT_IGNORE_CHARS: &[char] = &['[', ']', ',', '.', ' '];

/// Implements the basic algorithm used by the extractor
fn extractor<S: ToString, T: FromStr + ToString>(
    bp: char,
    data: &S,
    exclude: Option<&[char]>,
) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let data = data.to_string();
    let skip = exclude.unwrap_or(DEFAULT_IGNORE_CHARS);
    let trimmed: &str = data.trim_matches(skip);
    trimmed
        .split(bp)
        .map(|i| i.trim_matches(skip).parse::<T>().unwrap())
        .collect()
}
