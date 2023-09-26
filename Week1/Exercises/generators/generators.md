# Generators

Generators for the 2^32 roots of unity and for subgroups of order 2^16+1 and 257.  
Given, 7 is a generator of the multiplicative group of Zp\*, where p = 2^64 - 2^32 +1.

## Solution:

### Step 1: Compute the prime factorization of p = 2^64 - 2^32 + 1.

p = 2^64 - 2^32 + 1  
We can factor this using the difference of squares:  
p = (2^32 + 1)(2^32 - 1) + 1  
p = (2^32 + 1)(2^16 + 1)(2^16 - 1) + 1  
p = (2^32 + 1)(2^16 + 1)(2^8 + 1)(2^8 - 1) + 1  
p = (2^32 + 1)(2^16 + 1)(2^8 + 1)(2^4 + 1)(2^4 - 1) + 1  
p = (2^32 + 1)(2^16 + 1)(2^8 + 1)(17)(15) + 1

### Step 2: Compute generators for each of the factors: 2^32 + 1, 2^16 + 1, 2^8 + 1, 17, and 15.

_Using Primitive Root Theorem_  
_The Primitive Root Theorem is a fundamental concept in number theory and modular arithmetic. It states that for any prime number p, there exists at least one integer, called a primitive root modulo p, that generates the entire multiplicative group of integers modulo p._

- For 2^32 + 1 :: 7 - _mentioned_  
  According to the Primitive Root Theorem, there exists an integer (a primitive root) g such that g generates the entire multiplicative group modulo 2^32 + 1. You mentioned that 7 is a generator for this subgroup. This means that 7^k covers all non-zero residues modulo 2^32 + 1 for some positive integer k. It's worth noting that the choice of 7 as a generator may not be unique, and other generators could exist.

- For 2^16 + 1:: Let's call it g_16. If 7 is a generator of Zp\*, then 7^2^16 is a generator of the subgroup of order 2^16 + 1.
- For 2^8 + 1: Following the same logic, if 7 is a generator of Zp\*, then 7^2^16^2 is a generator of the subgroup of order 2^8 + 1.
- For 17 and 15 : Here, the Primitive Root Theorem doesn't guarantee the existence of a primitive root for all integers.  
  However, for small prime numbers like 17, we can find a generator by testing values.  
  So 3 for 17 and 2 for 15 fulfills the criteria of generators.

The Rust code calculates generators for specific subgroups within a multiplicative group modulo a large prime number p. The subgroups are defined by their order, and the code uses modular exponentiation and mathematical operations on BigUint values for precision.

### Functions

#### mod_pow

_mod_pow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint_  
Calculates base^exponent mod modulus using modular exponentiation.

Parameters:  
base: The base value as a BigUint.  
exponent: The exponent as a BigUint.  
modulus: The modulus as a BigUint.  
Returns: The result as a BigUint.

#### is_generator

_is_generator(base: &BigUint, modulus: &BigUint, subgroup_size: &BigUint) -> bool_  
Checks if base is a generator of a subgroup of order subgroup_size modulo modulus.

Parameters:  
base: The base value as a BigUint.  
modulus: The modulus as a BigUint  
subgroup_size: The order of the subgroup as a BigUint.  
Returns: true if base is a generator, false otherwise.

#### main

Defines the modulus p and the subgroup sizes
It calculates generators for the specified subgroups using mod_pow and is_generator.  
The results are printed to the console.

##### Notice: Ensure that the num-bigint and num-traits crates are included as dependencies in project's Cargo.toml file.
