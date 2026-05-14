/*
    Appellation: impl_convert <module>
    Contrib: @FL03
*/
use super::H256;

impl FromIterator<u8> for H256 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let digest = iter.into_iter().collect::<Vec<u8>>();
        crate::digest_to_hash::<32>(&digest).into()
    }
}

impl IntoIterator for H256 {
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, 32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<&T> for H256
where
    T: AsRef<[u8]>,
{
    fn from(data: &T) -> H256 {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].copy_from_slice(data.as_ref());
        H256(buffer)
    }
}

impl From<[u8; 32]> for H256 {
    fn from(input: [u8; 32]) -> H256 {
        H256(input)
    }
}

impl From<H256> for [u8; 32] {
    fn from(input: H256) -> [u8; 32] {
        input.0
    }
}

impl From<Vec<u8>> for H256 {
    fn from(input: Vec<u8>) -> H256 {
        crate::digest_to_hash::<32>(&input).into()
    }
}

impl From<H256> for Vec<u8> {
    fn from(input: H256) -> Vec<u8> {
        input.0.to_vec()
    }
}

impl From<crate::GenericHash> for H256 {
    fn from(data: crate::GenericHash) -> H256 {
        data.as_slice().to_owned().into()
    }
}

impl From<H256> for crate::GenericHash {
    fn from(input: H256) -> crate::GenericHash {
        crate::GenericHash::from(input.0)
    }
}

impl From<crate::hash::H160> for H256 {
    fn from(input: crate::hash::H160) -> H256 {
        let mut buffer = [0; 32];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}

impl From<H256> for crate::hash::H160 {
    fn from(input: H256) -> crate::hash::H160 {
        let mut buffer = [0; 20];
        buffer[..].copy_from_slice(&input.0[0..20]);
        buffer.into()
    }
}
