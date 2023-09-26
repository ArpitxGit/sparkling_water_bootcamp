extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive}; // Import the ToPrimitive trait

// Function to calculate a^b mod m for BigUint values
fn mod_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let one = BigUint::one();
    let mut result = one.clone();
    let mut base = base.clone() % modulus.clone();
    let mut exponent = exponent.clone();

    while exponent > one {
        if exponent.clone() % 2u8 == one.clone() {
            result = (result.clone() * base.clone()) % modulus.clone();
        }
        base = (base.clone() * base.clone()) % modulus.clone();
        exponent = exponent.clone() / 2u8;
    }

    result
}

// Function to check if a number is a generator of a subgroup
fn is_generator(base: &BigUint, modulus: &BigUint, subgroup_size: &BigUint) -> bool {
    let one = BigUint::one();

    let mut powers = vec![one.clone()];
    let mut current = base.clone();

    for _ in 1..subgroup_size.clone().to_u64().unwrap() {
        current = (current.clone() * base.clone()) % modulus.clone();
        powers.push(current.clone());
    }

    powers.sort();
    powers.dedup();

    powers.len() == subgroup_size.to_usize().unwrap()
}

fn main() {
    // Define the modulus p = 2^64 - 2^32 + 1 as large integer
    let p: BigUint = BigUint::from(2u64).pow(64) - BigUint::from(2u64).pow(32) + BigUint::one();

    // Define subgroup sizes
    let subgroup_size_1: BigUint = BigUint::from(2u64).pow(32);
    let subgroup_size_2: BigUint = BigUint::from(2u64).pow(16) + BigUint::one();
    let subgroup_size_3: BigUint = BigUint::from(257u32);

    // finding generators
    let generator_1: BigUint = BigUint::from(7u64);
    let generator_2: BigUint = mod_pow(&generator_1, &subgroup_size_1, &p);
    let generator_3: BigUint = mod_pow(&generator_1, &subgroup_size_2, &p);

    println!("Generator for the roots of 2^32 of unity: {}", generator_2);
    println!("Generator for a subgroup of order 2^16 + 1: {}", generator_3);

    // f inding a generator for a subgroup of order 257
    let mut potential_generator: BigUint = BigUint::from(2u64);
    while !is_generator(&potential_generator, &p, &subgroup_size_3) {
        potential_generator += BigUint::one();
    }
    println!("Generator for a subgroup of order 257: {}", potential_generator);
}
