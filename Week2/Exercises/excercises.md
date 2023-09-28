# Elliptic Curve Element Type (E)

An elliptic curve element type E consists of a pair of coordinates (x, y),  
where x and y are elements from the prime field 𝔽p.  
Additionally, we have the point at infinity 𝒪, representing the identity element.

The curve equation is defined as: y^2 ≡ x^3 + ax + b (mod p)  
Where a and b are constants specific to the chosen elliptic curve, and p is the prime field modulus.

## Basic Operations: Addition and Doubling

### Point Addition (P + Q):

So we have two elliptic curve elements:  
P = (x1, y1) and Q = (x2, y2),  
we can calculate the sum R = P + Q using the elliptic curve group law.  
This involves calculating s and x3 as follows:

- s = (y2 - y1) / (x2 - x1) (mod p)
- x3 = s^2 - x1 - x2 (mod p)
- y3 = s(x1 - x3) - y1 (mod p)  
  R = (x3, y3) is the result of point addition.  
  Ensure that the case when P = Q (doubling) is handled correctly.

### Point Doubling (2P):

Given an elliptic curve element P = (x1, y1), point doubling Q = 2P is calculated similarly to point addition, with s and x3 computed as follows:

- s = (3x1^2 + a) / (2y1) (mod p)
- x3 = s^2 - 2x1 (mod p)
- y3 = s(x1 - x3) - y1 (mod p)  
  Q = (x3, y3) is the result of point doubling.

## Scalar Multiplication

Scalar multiplication is the repeated addition of a point by itself a certain number of times.  
ex: , kP can be computed by adding P to itself k times. The result is kP.

## Subgroup Check

To check if a point P belongs to the correct subgroup of the elliptic curve, verify that P satisfies the curve equation (i.e., y^2 ≡ x^3 + ax + b (mod p)) and lies on the curve defined by the specific a, b, and p values.  
Additionally, check that P is not the point at infinity 𝒪 and that it has an order equal to the order of the subgroup.

## Subgroups and Attack

1. Let's consider the prime modulus p = 37,
2. Compute Euler's totient function phi(n), phi(37) = 36  
   This represents the total number of integers between 1 and 37 that are coprime to 37.
3. Find the divisors of 36. Divisors: 1, 2, 3, 4, 6, 9, 12, 18, 36.
4. For the sake of simplicity, we'll use an optimized trial division algorithm to find divisors for small numbers.
5. Find a generator point for the group modulo p = 37,
6. To find the generator(s), we follow these steps:

- Calculate the prime factors of phi(37),
  - phi(37) = 36 = 2^2 \* 3^3
- Finding potential primitive roots:
  - For each integer g from 2 to 36,
  - Compute g^(36/q) mod37 for each prime factor q of 36.
  - If any of these results in 1, then g is not a primitive root.
  - If none of them are 1, then g is a primitive root.
- Possible primitive roots: [2, 5, 13, 15, 17, 18, 19, 20, 22, 24, 29, 30, 32]

7. Compute subgroups of all orders:

- Choose a generator, for example, g = 5 for the full group (primitive root modulo 37).
- For each divisor d of 36, compute g^(36/d) mod 37 to get a generator for the subgroup of order d (divisors of 36).
- Compute subgroups of all orders using generators for each subgroup.
- We have the subgroups for p=37 are determined.

## Attack Using Subgroups:

This example illustrates how attackers can exploit the use of small subgroups in the Diffie-Hellman key exchange to determine a user's private key.

- Alice and Bob are using Diffie-Hellman key exchange over $Z_{37}^\star$ .
- They publicly agree on a generator g = 5 ,
- Eve, an attacker, wants to trick Alice into using the small subgroup of order 4.
- The generator of this subgroup is 7.
- Instead of sending a genuine public key, Eve sends 7 to Alice.
- Alice picks her private key k = 3 and computes the public key as:
  - K = 7^3 mod 37
- Eve knows Alice's public key K is in the subgroup of order 4 since Alice is using 7 for computations, resulting in K being an element of the subgroup {1, 7, 13, 19}.
- Eve computes powers of 7 mod 37 to find exponent 3:
  - 7^1 = 7mod37
  - 7^2 = 49 = 12mod37
  - 7^3 = 84 = 10mod37 (Match found).
- Hence, Eve now knows that Alice's private key is 3 with minimum effort.

Notice: In different system and setups, we need to perform different mathematical operations on deducing the subgroups resulting in a subgroup of a few elements, so as an attacker we can try with Brute Force trying to solve the hard problems with hit and trial in that subgraoup and bypass the security.
