/*
    Appellation: network_addr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "lowercase")
)]
pub struct NetworkAddr {
    /// the hostname or IP address
    pub(crate) host: String,
    /// the port number
    pub(crate) port: u16,
}

impl NetworkAddr {
    pub const LOCALHOST: &'static str = "0.0.0.0";
    pub(crate) const DEFAULT_PORT: u16 = 8080;

    pub fn new(host: impl ToString, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
    /// tries returning a new instance from the given [`url`](url::Url)
    #[cfg(feature = "url")]
    pub fn try_from_url(url: url::Url) -> Result<Self, crate::ConfigError> {
        let addr = Self {
            host: url
                .host_str()
                .ok_or(url::ParseError::EmptyHost)?
                .to_string(),
            port: url.port().ok_or(url::ParseError::InvalidPort)?,
        };
        Ok(addr)
    }
    #[cfg(feature = "url")]
    pub fn parse_url(url: &str) -> Result<Self, crate::ConfigError> {
        // parse the string into a URL
        let url = url::Url::parse(url)?;
        // try returning a new instance from the parsed URL
        Self::try_from_url(url)
    }
    /// returns a new instance with the given port and the localhost address
    pub fn localhost(port: u16) -> Self {
        Self::new(Self::LOCALHOST, port)
    }
    /// returns a new [`NetworkAddr`] instance from the given [`SocketAddr`](core::net::SocketAddr)
    pub fn from_socket_addr(addr: core::net::SocketAddr) -> Self {
        Self {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }
    /// parse the address into a [`SocketAddr`](core::net::SocketAddr)
    pub fn as_socket_addr(&self) -> core::net::SocketAddr {
        self.to_string().parse().unwrap()
    }
    /// parse the hostname into an [`IpAddr`](core::net::IpAddr)
    pub fn ip(&self) -> core::net::IpAddr {
        self.host().parse().unwrap()
    }
    /// returns a reference to the hostname
    pub fn host(&self) -> &str {
        &self.host
    }
    /// returns a reference to the port
    pub fn port(&self) -> u16 {
        self.port
    }
    /// updates the port before returning a mutable reference to the address
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    /// consumes the current instance to create another with the given port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    /// updates the host before returning a mutable reference to the address
    pub fn set_host<T: ToString>(&mut self, host: T) -> &mut Self {
        self.host = host.to_string();
        self
    }
    /// consumes the current instance to create another with the given host
    pub fn with_host<T: ToString>(self, host: T) -> Self {
        Self {
            host: host.to_string(),
            ..self
        }
    }
}

impl Default for NetworkAddr {
    fn default() -> Self {
        Self::localhost(NetworkAddr::DEFAULT_PORT)
    }
}

impl core::fmt::Display for NetworkAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{host}:{port}", host = self.host, port = self.port)
    }
}

impl core::str::FromStr for NetworkAddr {
    type Err = crate::ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[cfg(feature = "url")]
        {
            let url = url::Url::parse(s)?;
            url.try_into()
        }
        #[cfg(not(feature = "url"))]
        {
            let mut parts = s.split(':');
            let host: core::net::IpAddr = parts.next().unwrap().parse()?;
            let port = parts.next().unwrap().parse()?;
            Ok(Self::new(host, port))
        }
    }
}

impl From<core::net::SocketAddr> for NetworkAddr {
    fn from(addr: core::net::SocketAddr) -> Self {
        Self::from_socket_addr(addr)
    }
}

impl From<NetworkAddr> for core::net::SocketAddr {
    fn from(addr: NetworkAddr) -> Self {
        addr.as_socket_addr()
    }
}

#[cfg(feature = "url")]
impl TryFrom<url::Url> for NetworkAddr {
    type Error = crate::ConfigError;

    fn try_from(url: url::Url) -> Result<Self, Self::Error> {
        let host = url.host_str().ok_or(url::ParseError::EmptyHost)?;
        let port = url.port().ok_or(url::ParseError::InvalidPort)?;
        Ok(Self::new(host, port))
    }
}
