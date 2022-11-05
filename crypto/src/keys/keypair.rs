/*
    Appellation: keypair <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::keys::generate_random_pkcs8;
use ring::{pkcs8, rand::SystemRandom, signature::Ed25519KeyPair};

#[derive(Debug)]
pub struct Keypair(Ed25519KeyPair);

impl Keypair {
    pub fn new(data: Ed25519KeyPair) -> Self {
        Self(data)
    }
    pub fn generate() -> Self {
        Self::from(generate_random_pkcs8())
    }
    pub fn keypair(&self) -> &Ed25519KeyPair {
        &self.0
    }
}

impl std::convert::From<&[u8]> for Keypair {
    fn from(data: &[u8]) -> Self {
        Self::new(Ed25519KeyPair::from_pkcs8(data).expect(""))
    }
}

impl std::convert::From<pkcs8::Document> for Keypair {
    fn from(data: pkcs8::Document) -> Self {
        Self::from(data.as_ref())
    }
}

#[derive(Debug)]
pub enum Keys {
    ED25519(Ed25519KeyPair),
}

impl Keys {
    pub fn new() -> Self {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).ok().unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .ok()
            .unwrap();
        Self::ED25519(keypair)
    }
}

impl Default for Keys {
    fn default() -> Self {
        Self::new()
    }
}
