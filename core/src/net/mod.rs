/*
    Appellation: net <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::utils::*;

pub(crate) mod utils {
    use core::net::{self, SocketAddr};
    /// This function attempts to convert the given input into a [core::net::SocketAddr]
    pub fn try_str_to_socketaddr(addr: impl ToString) -> Result<SocketAddr, net::AddrParseError> {
        addr.to_string().parse()
    }
}

pub(crate) mod prelude {
    pub use super::utils::*;
}
