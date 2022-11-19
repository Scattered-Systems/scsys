/*
    Appellation: crypto <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{keypair::*, public_key::*, secret_key::*, utils::*};

pub(crate) mod keypair;
pub(crate) mod public_key;
pub(crate) mod secret_key;

pub(crate) mod utils {
    use ring::{rand::SystemRandom, signature::Ed25519KeyPair};

    /// Generate a random key pair.
    pub fn random_keypair() -> Ed25519KeyPair {
        Ed25519KeyPair::from_pkcs8(generate_random_pkcs8().as_ref()).unwrap()
    }

    pub fn generate_random_pkcs8() -> ring::pkcs8::Document {
        Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap()
    }
}
