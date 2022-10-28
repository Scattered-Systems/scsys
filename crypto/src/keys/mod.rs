/*
    Appellation: crypto <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::keypair::*;

pub(crate) mod keypair;

pub(crate) mod utils {
    use ring::{rand, signature::Ed25519KeyPair};

    /// Generate a random key pair.
    pub fn random_keypair() -> Ed25519KeyPair {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref().into()).unwrap()
    }
}
