use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::thread_rng;
use rand_core::OsRng;
use rand_core::RngCore;

fn main() {
    // Initialize secp256k1 context
    let secp = Secp256k1::new();

    // Generate a random private key
    let mut rng = OsRng;
    let mut data = [0u8; 32];
    rng.fill_bytes(&mut data);
    let secret_key = SecretKey::from_slice(&data).expect("Invalid secret key");

    // Derive the corresponding public key
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Serialize the public key into compressed form
    let compressed_public_key = public_key.serialize();

    // Deserialize the compressed public key
    let deserialized_public_key = PublicKey::from_slice(&compressed_public_key).unwrap();

    // Check if the deserialized public key matches the original public key
    assert_eq!(public_key, deserialized_public_key);

    println!("Original Public Key: {:?}", public_key);
    println!("Compressed Public Key: {:?}", compressed_public_key);
    println!("Deserialized Public Key: {:?}", deserialized_public_key);
}

#[cfg(test)]
mod tests {
    use secp256k1::{Secp256k1, SecretKey, PublicKey};
    use rand::thread_rng;
    use rand_core::OsRng;
    use rand_core::RngCore;

    #[test]
    fn test_point_compression_decompression() {
        // Initialize secp256k1 context
        let secp = Secp256k1::new();

        // Generate a random private key
        let mut rng = OsRng;
        let mut data = [0u8; 32];
        rng.fill_bytes(&mut data);
        let secret_key = SecretKey::from_slice(&data).expect("Invalid secret key");

        // Derive the corresponding public key
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // Serialize the public key into compressed form
        let compressed_public_key = public_key.serialize();

        // Deserialize the compressed public key
        let deserialized_public_key = PublicKey::from_slice(&compressed_public_key).unwrap();

        // Check if the deserialized public key matches the original public key
        assert_eq!(public_key, deserialized_public_key);
    }
}
