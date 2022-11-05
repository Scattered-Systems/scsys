/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{h160::H160, h256::H256, utils::*};

pub(crate) mod h160;
pub(crate) mod h256;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Hashes {
    H256(H256),
    H160(H160),
}

impl Default for Hashes {
    fn default() -> Self {
        Self::H256(H256::default())
    }
}

pub(crate) mod utils {
    use super::H256;
    use rand::Rng;

    pub fn generate_random_hash() -> H256 {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let mut raw_bytes = [0; 32];
        raw_bytes.copy_from_slice(&random_bytes);
        (&raw_bytes).into()
    }
}
