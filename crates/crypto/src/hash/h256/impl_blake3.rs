use super::H256;
use crate::Concat;

use blake3::Hash;

impl Concat<H256> for H256 {
    type Output = H256;

    fn concat(&self, rhs: H256) -> Self {
        let mut curr = self.to_vec();
        // append the given hash to the current hash
        curr.append(&mut rhs.to_vec());

        blake3::hash(&curr).into()
    }
}

impl From<Hash> for H256 {
    fn from(input: Hash) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H256(raw_hash)
    }
}

impl From<H256> for Hash {
    fn from(input: H256) -> Hash {
        Hash::from(input.0)
    }
}
