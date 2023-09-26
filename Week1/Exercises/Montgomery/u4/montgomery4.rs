fn montgomery_multiply(a: u8, b: u8, m: u8) -> u8 {
  let n = 4; // Number of bits
  let r = 1 << n; // R = 2^n, where n is the number of bits
  let r_inv = 9; // The modular inverse of R modulo m. In this case, 9 is the modular inverse of 16 modulo 15.

  // Step 1: Convert a and b to Montgomery form
  let a_montgomery = (a as u16) * (r as u16) % (m as u16);
  let b_montgomery = (b as u16) * (r as u16) % (m as u16);

  // Step 2: Initialize the result and carry
  let mut result = 0_u16;
  let mut carry = 0_u16;

  // Step 3: Perform Montgomery multiplication
  for i in 0..n {
      let a_i = (a_montgomery >> i) & 1;
      if a_i == 1 {
          // If the current bit of a is 1, add b_montgomery to the result
          result += b_montgomery;
      }

      // Check if the least significant bit of the result is 1
      let lsb = result & 1;

      // Calculate the carry for the next iteration
      carry = (carry + (lsb * m as u16)) >> 1;

      // Right-shift the result by 1
      result >>= 1;
  }

  // Step 4: Final conversion from Montgomery form
  if result >= (m as u16) {
      result -= (m as u16);
  }

  result as u8
}

fn main() {
  let a = 7_u8; // 4-bit integer A
  let b = 5_u8; // 4-bit integer B
  let m = 15_u8; // 4-bit modulus M

  let result = montgomery_multiply(a, b, m);
  println!("{} * {} mod {} = {}", a, b, m, result);
}