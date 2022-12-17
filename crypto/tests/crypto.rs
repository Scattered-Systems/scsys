#[cfg(test)]
mod tests {

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
        let pi = vrf.prove(&secret_key, message).unwrap();
        let hash = vrf.proof_to_hash(&pi).unwrap();
        println!("Generated VRF proof: {}", hex::encode(&pi));

        // VRF proof verification (returns VRF hash output)
        let beta = vrf.verify(&public_key, &pi, message);

        match beta {
            Ok(beta) => {
                println!("VRF proof is valid!\nHash output: {}", hex::encode(&beta));
                assert_eq!(hash, beta);
            }
            Err(e) => {
                println!("VRF proof is not valid: {e}");
            }
        }
    }
}
