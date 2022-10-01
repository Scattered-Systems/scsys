/*
    Appellation: hash <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{
    h160::H160,
    h256::{generate_random_hash, hash_divide_by, hash_multiply_by, H256},
};

mod h160;
mod h256;

use rand::Rng;
use serde::{Deserialize, Serialize};

pub type H256Hash = [u8; 32];
pub type H160Hash = [u8; 20];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Hashes {
    H256(H256),
    H160(H160),
}

pub trait Hashable {
    fn hash(&self) -> H256;
    fn gen_rand() -> H256 {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::{hash_multiply_by, H256};

    use vrf::openssl::{CipherSuite, ECVRF};
    use vrf::VRF;

    #[test]
    fn vrf_test() {
        let mut vrf = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        // Inputs: Secret Key, Public Key (derived) & Message
        let secret_key =
            hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721")
                .unwrap();
        let public_key = vrf.derive_public_key(&secret_key).unwrap();
        let message: &[u8] = b"sample";

        // VRF proof and hash output
        let pi = vrf.prove(&secret_key, &message).unwrap();
        let hash = vrf.proof_to_hash(&pi).unwrap();
        println!("Generated VRF proof: {}", hex::encode(&pi));

        // VRF proof verification (returns VRF hash output)
        let beta = vrf.verify(&public_key, &pi, &message);

        match beta {
            Ok(beta) => {
                println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
                assert_eq!(hash, beta);
            }
            Err(e) => {
                println!("VRF proof is not valid: {}", e);
            }
        }
    }
    #[test]
    fn test_hash_multiply() {
        let hash: H256 = [2u8; 32].into();
        let result = hash_multiply_by(&hash, 0.5f64);
        let halfhash: H256 = [1u8; 32].into();
        assert_eq!(result, halfhash);
    }
}
