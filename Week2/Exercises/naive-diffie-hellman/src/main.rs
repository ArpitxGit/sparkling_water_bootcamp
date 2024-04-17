use num_bigint::{BigUint, RandBigInt};
use rand::rngs::OsRng;

fn main() {
    // Known large prime p (a safe prime for Diffie-Hellman)
    let p = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFC90FDAA22168C234C4C6628B80DC1CD129024E088A67CC74020BBEA63B139B22514A08798E3404DDEF9519B3CD3A431B302B0A6DF25F14374FE1356D6D51C245E485B576625E7EC6F44C42E9A637ED6B0BFF5CB6F406B7EDEE386BFB5A899FA5AE9F24117C4B1FE649286651ECE65381FFFFFFFFFFFFFFFF",
        16,
    )
    .expect("Invalid prime");
    let g = BigUint::from(2u32);

    let mut rng = OsRng;

    // Alice generates her keys
    let a = rng.gen_biguint_below(&p);
    let a_pub = g.modpow(&a, &p);

    // Validate public key
    if a_pub.is_zero() || a_pub == BigUint::from(1u32) || a_pub >= p {
        panic!("Invalid public key generated");
    }

    // Bob generates his keys
    let b = rng.gen_biguint_below(&p);
    let b_pub = g.modpow(&b, &p);

    // Validate public key
    if b_pub.is_zero() || b_pub == BigUint::from(1u32) || b_pub >= p {
        panic!("Invalid public key generated");
    }

    // Compute the shared secret
    let alice_secret = b_pub.modpow(&a, &p);
    let bob_secret = a_pub.modpow(&b, &p);

    // Check if secrets match
    assert_eq!(alice_secret, bob_secret);
    println!("Shared secret is: {}", alice_secret);
}
