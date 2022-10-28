/*
    Appellation: hashes <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{h160::H160, h256::H256, utils::*};

pub(crate) mod h160;
pub(crate) mod h256;

pub trait Hashable {
    fn hash(&self) -> H256;
    fn generate(&self) -> H256 {
        generate_random_hash()
    }
}

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

    pub fn hash_divide_by(input: &H256, divide: f64) -> H256 {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
            //println!{"{}",value};
            let value = value as f64;
            let result = value / divide;
            let result = result as u32;
            let results: [u8; 4] = result.to_be_bytes();
            //println!{"{}",result};
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        (&result_bytes).into()
    }

    pub fn hash_multiply_by(input: &H256, multiply: f64) -> H256 {
        let mut result_bytes = [0; 32];
        for n in 1..9 {
            let value = u32::from_be_bytes(input.0[4 * (n - 1)..4 * n].try_into().unwrap());
            //println!{"{}",value};
            let value = value as f64;
            let result = value * multiply;
            let result = result as u32;
            let results: [u8; 4] = result.to_be_bytes();
            //println!{"{}",result};
            result_bytes[4 * (n - 1)] = results[0];
            result_bytes[4 * (n - 1) + 1] = results[1];
            result_bytes[4 * (n - 1) + 2] = results[2];
            result_bytes[4 * (n - 1) + 3] = results[3];
        }
        (&result_bytes).into()
    }
}
