use super::H160;
use crate::Concat;

impl Concat for H160 {
    type Output = H160;

    fn concat(&self, other: H160) -> Self {
        let mut res: Vec<u8> = (*self).into();
        let mut rnode: Vec<u8> = (*other).into();
        res.append(&mut rnode);

        blake3::hash(&res).into()
    }
}

impl From<blake3::Hash> for H160 {
    fn from(input: blake3::Hash) -> H160 {
        let mut raw_hash: [u8; 20] = [0; 20];
        raw_hash[0..32].copy_from_slice(input.as_bytes());
        H160(raw_hash)
    }
}

// impl From<H160> for blake3::Hash {
//     fn from(input: H160) -> blake3::Hash {
//         let hash: [u8; 32] = blake3::Hash::from(input.into::<>());

//     }
// }
