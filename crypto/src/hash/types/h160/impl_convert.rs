/*
    Appellation: impl_convert <module>
    Contrib: @FL03
*/
use super::H160;
use crate::GenericHash;

impl FromIterator<u8> for H160 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let digest = iter.into_iter().collect::<Vec<u8>>();
        crate::digest_to_hash::<20>(&digest).into()
    }
}

impl IntoIterator for H160 {
    type Item = u8;
    type IntoIter = std::array::IntoIter<u8, 20>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> From<&T> for H160
where
    T: AsRef<[u8]>,
{
    fn from(data: &T) -> H160 {
        let mut buffer: [u8; 20] = [0; 20];
        buffer[..].copy_from_slice(data.as_ref());
        H160(buffer)
    }
}

impl From<[u8; 20]> for H160 {
    #[inline]
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl From<Vec<u8>> for H160 {
    fn from(input: Vec<u8>) -> H160 {
        crate::digest_to_hash::<20>(&input).into()
    }
}

impl From<GenericHash> for H160 {
    fn from(data: GenericHash) -> H160 {
        data.as_slice().to_owned().into()
    }
}

impl From<H160> for [u8; 20] {
    #[inline]
    fn from(value: H160) -> Self {
        value.0
    }
}

impl<'a> From<&'a H160> for &'a [u8; 20] {
    #[inline]
    fn from(value: &'a H160) -> Self {
        &value.0
    }
}

impl From<H160> for Vec<u8> {
    fn from(input: H160) -> Vec<u8> {
        input.0.to_vec()
    }
}
