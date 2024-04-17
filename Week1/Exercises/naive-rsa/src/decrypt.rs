/*
decrypt function takes a private key and a ciphertext as input.
It utilizes the num_bigint crate to handle large integer arithmetic and the base64 crate to decode the private key.
The function first decodes the private key from Base64 encoding and parses it into d (private exponent) and n (modulus).
It then parses the ciphertext and modulus into c and n, respectively.
Using the RSA decryption algorithm, it computes the plaintext message m by raising the ciphertext to the power of the private exponent modulo n.
*/

use num_bigint::BigUint;
use base64::{Engine as _, engine::general_purpose};



pub fn decrypt(private_key: &str, c: &str){


    let private_key = general_purpose::STANDARD_NO_PAD.decode(private_key).unwrap();

    let private_key_str = std::str::from_utf8(&private_key).expect("Invalid UTF-8 sequence");

    let mut keys = private_key_str.split(';');
    let d = keys.next().unwrap_or_default();
    let n = keys.next().unwrap_or_default();


    let d = BigUint::parse_bytes(d.as_bytes(), 10).expect("Invalid number");
    let c = BigUint::parse_bytes(c.as_bytes(), 10).expect("Invalid number");
    let n = BigUint::parse_bytes(n.as_bytes(), 10).expect("Invalid number");

    let m = c.modpow(&d, &n);
    let bytes= m.to_bytes_be();

    let recovered_message = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");

    println!("Your message is: {}", recovered_message);
}