/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::NetworkAddr;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct NetworkConfig {
    pub(crate) address: NetworkAddr,
    pub(crate) basepath: Option<String>,
    pub(crate) max_connections: Option<u16>,
    pub(crate) open: bool,
}

impl NetworkConfig {
    pub fn new() -> Self {
        Self {
            address: NetworkAddr::default(),
            basepath: None,
            max_connections: None,
            open: false,
        }
    }
    /// toggle the open on startup flag
    pub fn open_on_startup(self) -> Self {
        Self { open: true, ..self }
    }
    /// returns an immutable reference to the network address
    pub const fn address(&self) -> &NetworkAddr {
        &self.address
    }
    /// returns a mutable reference to the network address
    pub const fn address_mut(&mut self) -> &mut NetworkAddr {
        &mut self.address
    }
    /// returns a reference to the basepath; if any.
    pub fn basepath(&self) -> Option<&String> {
        self.basepath.as_ref()
    }
    /// returns a mutable reference to the basepath; if any.
    pub fn basepath_mut(&mut self) -> Option<&mut String> {
        self.basepath.as_mut()
    }
    /// returns the maximum number of connections; if any.
    pub fn max_connections(&self) -> Option<u16> {
        self.max_connections
    }
    /// returns true if the application should automatically open the browser upon startup
    pub fn open(&self) -> bool {
        self.open
    }
    /// converts the network address into a [`SocketAddr`](core::net::SocketAddr)
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        // self.address.into_iter().map(|i| i.as_socket_addr())
        self.address().as_socket_addr()
    }
    /// Returns the host of the address
    pub fn host(&self) -> &str {
        self.address().host()
    }
    /// Returns the ip of the address
    pub fn ip(&self) -> core::net::IpAddr {
        self.address().ip()
    }
    /// Returns the port of the address
    pub fn port(&self) -> u16 {
        self.address().port()
    }
    /// update the address then return a mutable reference to the current instance
    pub fn set_address(&mut self, address: NetworkAddr) -> &mut Self {
        self.address = address;
        self
    }
    /// consumes the current instance to create another with the given address
    pub fn with_address(self, address: NetworkAddr) -> Self {
        Self { address, ..self }
    }
    /// update the basepath then return a mutable reference to the current instance
    pub fn set_basepath<T: ToString>(&mut self, basepath: T) -> &mut Self {
        self.basepath = Some(basepath.to_string());
        self
    }
    /// consumes the current instance to create another with the given basepath
    pub fn with_basepath<T: ToString>(self, basepath: T) -> Self {
        Self {
            basepath: Some(basepath.to_string()),
            ..self
        }
    }
    /// update the max_connections then return a mutable reference to the current instance
    pub fn set_max_connections(&mut self, max_connections: u16) -> &mut Self {
        self.max_connections = Some(max_connections);
        self
    }
    /// consumes the current instance to create another with the given max_connections
    pub fn with_max_connections(self, max_connections: u16) -> Self {
        Self {
            max_connections: Some(max_connections),
            ..self
        }
    }
    /// update the open flag then return a mutable reference to the current instance
    pub fn set_open(&mut self, open: bool) -> &mut Self {
        self.open = open;
        self
    }
    /// consumes the current instance to create another with the given open flag
    pub fn with_open(self, open: bool) -> Self {
        Self { open, ..self }
    }
    /// updates the hostname of the current address
    pub fn set_host<T: ToString>(&mut self, host: T) -> &mut Self {
        self.address_mut().set_host(host);
        self
    }
    /// update the host then return a mutable reference to the current instance
    pub fn with_host<T: ToString>(self, host: T) -> Self {
        Self {
            address: self.address.with_host(host),
            ..self
        }
    }
    /// update the port then return a mutable reference to the current instance
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.address_mut().set_port(port);
        self
    }
    /// consumes the current instance to create another with the given port
    pub fn with_port(self, port: u16) -> Self {
        Self {
            address: NetworkAddr {
                port,
                ..self.address
            },
            ..self
        }
    }
}
