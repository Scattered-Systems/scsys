/*
    Appellation: keypair <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use ring::{rand::SystemRandom, signature::Ed25519KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PublicKey<T>(T);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SecretKey<T>(T);

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
