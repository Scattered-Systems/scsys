/*
    Appellation: networks <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, crate::Deserialize, crate::Serialize)]
pub enum DigitalAddress {
    Blockchain(String),
    Email(String),
    IPFS(String),
    Web3(String),
}

#[derive(Clone, Debug, Hash, PartialEq, crate::Deserialize, crate::Serialize)]
pub enum MachineAddress {
    Ip(std::net::IpAddr),
    Socket(std::net::SocketAddr),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
