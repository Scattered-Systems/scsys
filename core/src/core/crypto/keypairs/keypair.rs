/*
    Appellation: keypair <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::Timestamp;
use ring::signature::{EcdsaKeyPair, Ed25519KeyPair};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Keypair {
    ED25519(Ed25519KeyPair),
}

impl Keypair {
    pub fn new() -> Self {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).ok().unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .ok()
            .unwrap();
        let timestamp = Timestamp::default();
        Self::ED25519(keypair)
    }
}
