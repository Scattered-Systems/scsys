use super::H256;
use crate::Concat;

impl Concat for H256 {
    type Output = H256;

    fn concat(&self, other: H256) -> Self {
        let mut res: Vec<u8> = (*self).into();
        let mut rnode: Vec<u8> = (*other).into();
        res.append(&mut rnode);

        blake3::hash(&res).into()
    }
}

impl From<blake3::Hash> for H256 {
    fn from(input: blake3::Hash) -> H256 {
        let mut raw_hash: [u8; 32] = [0; 32];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H256(raw_hash)
    }
}

impl From<H256> for blake3::Hash {
    fn from(input: H256) -> blake3::Hash {
        blake3::Hash::from(input.0)
    }
}
