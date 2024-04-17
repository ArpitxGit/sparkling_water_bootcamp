/*
utilities
The modinv function efficiently computes the modular inverse of a modulo m.
It iterates through the extended Euclidean algorithm until a becomes one.

The find_miller_rabbin_coefficient function determines the value of m for the Miller-Rabin primality test,
finding the largest divisor of n - 1 that is not divisible by 2.

The is_prime function implements the Miller-Rabin primality test to determine
if a given BigInt n is likely prime. It handles trivial cases and then calculates m.
It then performs modular exponentiation using a fixed base a to verify the primality of n.
*/

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::identities::{Zero, One};

//faster modular inverse impl from https://www.reddit.com/r/rust/comments/cwbxxy/how_to_find_multiplicative_inverse_of_a_bigint/
pub fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
    if m0 == BigInt::one() {return BigInt::one()}

    let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), BigInt::zero(), BigInt::one());
    
    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x0;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
    }
    if inv < BigInt::zero() { inv += m0 }
    inv
}

fn find_miller_rabbin_coefficient(n: &BigInt) -> BigInt{

    let mut k = 1;
    let mut m = BigInt::from(0);

    loop {
        let power = BigInt::from(2).pow(k);
        let reminder = n % &power;
        if reminder != BigInt::from(0) {
            return m;
        }
        m = n / &power;
        k += 1;
    }

}

//Using Miller-Rabin primality test
pub fn is_prime(n: &BigInt) -> bool {

    // trivial prime cases
    if *n == BigInt::from(1) || *n == BigInt::from(0) {return false}
    if *n == BigInt::from(2) || *n == BigInt::from(3) {return true}

    // Step one, find n-1 = 2^k * m
    let n_minus_one = n - BigInt::from(1);
    let m: BigInt = find_miller_rabbin_coefficient(&n_minus_one);

    // 1 < a < n-1
    let a = BigInt::from(2);

    let mut b: BigInt = a.modpow(&m, n);

    while b != BigInt::from(1) && b != (n - &BigInt::from(1)) {
        b = b.modpow(&BigInt::from(2), n);
        break;
    }

    b == (n - &BigInt::from(1))    
}