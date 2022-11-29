/*
    Appellation: proxy <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Connection {
    Incoming(SocketAddr),
    Outgoing(SocketAddr),
}

impl Default for Connection {
    fn default() -> Self {
        Self::Outgoing(SocketAddr::from(([0, 0, 0, 0], 8080)))
    }
}
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]

pub struct ReverseProxy {
    pub connections: Vec<Connection>,
}
