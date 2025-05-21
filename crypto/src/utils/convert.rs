/*
    Appellation: convert <module>
    Contrib: @FL03
*/

pub fn digest_to_hash<const N: usize>(hash: impl AsRef<[u8]>) -> [u8; N] {
    let mut raw_hash: [u8; N] = [0; N];
    raw_hash[0..N].copy_from_slice(hash.as_ref());
    raw_hash
}
