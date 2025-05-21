/*
    Appellation: endpoint <module>
    Contrib: @FL03
*/
/// [`Endpoint`] is a general purpose networking address structure that is more suitable for
/// use in configuration files.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Endpoint {
    /// the hostname of IP address
    pub(crate) host: String,
    /// the port number
    pub(crate) port: u16,
}

impl Endpoint {
    /// the standard localhost address
    const LOCALHOST: &'static str = "0.0.0.0";
    /// the default port to fallback to if none is provided
    const DEFAULT_PORT: u16 = 8080;

    /// creates a new instance of [`Endpoint`] with the given host and port.
    pub fn create(host: impl ToString, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
    /// returns a new instance of [`Endpoint`] with default values.
    pub fn new() -> Self {
        Self::localhost(Self::DEFAULT_PORT)
    }
    /// returns a new address pre-configured to bind to localhost at the given port.
    pub fn localhost(port: u16) -> Self {
        Self {
            host: Self::LOCALHOST.to_string(),
            port,
        }
    }
    /// attempts to create a new instance of [`Endpoint`] from the environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("APP_HOST").unwrap_or_else(|_| Self::LOCALHOST.to_string());
        let port = std::env::var("APP_HOST")
            .ok()
            .map(|i| i.parse::<u16>().ok())
            .flatten()
            .unwrap_or(Self::DEFAULT_PORT);
        let addr = Self::create(host, port);
        Ok(addr)
    }
    /// returns a new instance given a host as a slice and some port
    pub fn from_host_slice_with_port(host: &[u8], port: u16) -> Self {
        let tmph = host
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<_>>()
            .join(".");
        let host_ip: core::net::IpAddr = tmph.parse().unwrap();
        Self {
            host: host_ip.to_string(),
            port,
        }
    }
    /// create a new instance of [`Endpoint`] from a [`SocketAddr`](core::net::SocketAddr).
    pub fn from_socket_addr(addr: core::net::SocketAddr) -> Self {
        Self {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }
    /// returns an immutable reference to the hostname
    pub fn host(&self) -> &str {
        &self.host
    }
    #[inline]
    /// returns a mutable reference to the hostname
    pub fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }
    #[inline]
    /// converts the hostname as an [`IpAddr`](core::net::IpAddr)
    pub fn host_as_ip(&self) -> core::net::IpAddr {
        self.host().parse().unwrap()
    }
    /// returns a copy of the endpoint port
    pub const fn port(&self) -> u16 {
        self.port
    }
    #[inline]
    /// returns a mutable reference to the port
    pub fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }
    /// use the [`replace`](core::mem::replace) method to exchange the current host with the
    /// given value, return the previous entry in the process.
    pub fn replace_host(&mut self, host: impl ToString) -> String {
        core::mem::replace(self.host_mut(), host.to_string())
    }
    /// use the [`replace`](core::mem::replace) method to exchange the current port with the
    /// given value, return the previous entry in the process.
    pub fn replace_port(&mut self, port: u16) -> u16 {
        core::mem::replace(self.port_mut(), port)
    }
    #[inline]
    /// sets the hostname to the given value
    pub fn set_host<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.host_mut() = value.to_string();
        self
    }
    #[inline]
    /// sets the port to the given value
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        *self.port_mut() = port;
        self
    }
    #[inline]
    /// consumes the current instance to return another configured with the given host.
    pub fn with_host<U: ToString>(self, value: U) -> Self {
        Self {
            host: value.to_string(),
            ..self
        }
    }
    #[inline]
    /// consumes the current instance to return another configured with the given port.
    pub fn with_port(self, port: u16) -> Self {
        Self { port, ..self }
    }
    /// returns the endpoint as a [`SocketAddr`](core::net::SocketAddr).
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        format!("{}:{}", self.host, self.port).parse().unwrap()
    }
    /// returns the hostname as an [`IpAddr`](core::net::IpAddr).
    pub fn ip(&self) -> core::net::IpAddr {
        self.as_socket_addr().ip()
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Self::create(Endpoint::LOCALHOST.to_string(), Endpoint::DEFAULT_PORT)
    }
}

impl core::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

unsafe impl core::marker::Send for Endpoint {}

unsafe impl core::marker::Sync for Endpoint {}

impl core::str::FromStr for Endpoint {
    type Err = core::net::AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let addr = s.parse::<core::net::SocketAddr>()?;
        Ok(Self::from_socket_addr(addr))
    }
}

impl From<core::net::SocketAddr> for Endpoint {
    fn from(addr: core::net::SocketAddr) -> Self {
        Self::create(addr.ip().to_string(), addr.port())
    }
}

impl From<Endpoint> for core::net::SocketAddr {
    fn from(config: Endpoint) -> Self {
        config.as_socket_addr()
    }
}
