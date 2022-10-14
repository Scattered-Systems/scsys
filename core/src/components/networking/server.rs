/*
    Appellation: servers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::extract::Extractor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub fn address(self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(self.pieces())
    }
    pub fn pieces(self) -> ([u8; 4], u16) {
        let host: [u8; 4] = Extractor::new('.', self.host.clone())
            .extract()
            .try_into()
            .ok()
            .unwrap();
        (host, self.port)
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
