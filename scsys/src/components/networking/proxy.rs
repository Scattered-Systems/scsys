/*
    Appellation: proxy <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use std::net::SocketAddr;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Connection {
    Incoming(SocketAddr),
    Outgoing(SocketAddr),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ReverseProxy {
    pub incomming: std::net::SocketAddr,
}
