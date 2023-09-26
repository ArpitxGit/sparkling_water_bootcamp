use super::element::UnsignedInteger;

pub struct MontgomeryAlgorithms;

impl MontgomeryAlgorithms {
    /// Performing Coarsely Integrated Operand Scanning (CIOS) Montgomery multiplication
    /// for 32-bit fields.
    ///
    /// # Parameters
    ///
    /// - `a`: The first operand as an `UnsignedInteger`.
    /// - `b`: The second operand as an `UnsignedInteger`.
    /// - `q`: The modulus as an `UnsignedInteger`.
    /// - `mu`: The inverse of `-q modulo 2^32` as a `u32`.
    ///
    /// # Returns
    ///
    /// A new `UnsignedInteger` containing the result of the multiplication.
    #[inline(always)]
    pub const fn cios<const NUM_LIMBS: usize>(
        a: &UnsignedInteger<NUM_LIMBS>,
        b: &UnsignedInteger<NUM_LIMBS>,
        q: &UnsignedInteger<NUM_LIMBS>,
        mu: &u32,
    ) -> UnsignedInteger<NUM_LIMBS> {
        // Initialize temporary arrays and variables.
        let mut t = [0_u32; NUM_LIMBS];
        let mut t_extra = [0_u32; 2];

        // Loop through the limbs of the result.
        let mut i: usize = NUM_LIMBS;
        while i > 0 {
            i -= 1;
            let mut c: u64 = 0; // Carry initialization.

            // Perform CIOS multiplication.
            let mut cs: u64;
            let mut j: usize = NUM_LIMBS;
            while j > 0 {
                j -= 1;
                cs = t[j] as u64 + (a.limbs[j] as u64) * (b.limbs[i] as u64) + c;
                c = cs >> 32; // Carry propagation.
                t[j] = cs as u32; // Update the result limbs.
            }

            // Update temporary variables.
            cs = (t_extra[1] as u64) + c;
            t_extra[0] = (cs >> 32) as u32;
            t_extra[1] = cs as u32;

            // Reset carry for the next step.
            let mut c: u64;

            // Calculate the multiplier `m`.
            let m = ((t[NUM_LIMBS - 1] as u64 * *mu as u64) << 32) >> 32;

            // Calculate carry and update the result limbs.
            c = (t[NUM_LIMBS - 1] as u64 + m * (q.limbs[NUM_LIMBS - 1] as u64)) >> 32;

            let mut j: usize = NUM_LIMBS - 1;
            while j > 0 {
                j -= 1;
                cs = t[j] as u64 + m * (q.limbs[j] as u64) + c;
                c = cs >> 32;
                t[j + 1] = ((cs << 32) >> 32) as u32;
            }

            // Update temporary variables.
            cs = (t_extra[1] as u64) + c;
            c = cs >> 32;
            t[0] = ((cs << 32) >> 32) as u32;

            t_extra[1] = t_extra[0] + c as u32;
        }

        // Create a new `UnsignedInteger` with the result.
        let mut result = UnsignedInteger { limbs: t };

        // Check for overflow and perform subtraction if necessary.
        let overflow = t_extra[0] > 0;
        if overflow || UnsignedInteger::const_le(q, &result) {
            (result, _) = UnsignedInteger::sub(&result, q);
        }

        // Return the final result.
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::unsigned_integer::element::UnsignedInteger;
    use crate::montgomery::MontgomeryAlgorithms;

    #[test]
    fn montgomery_multiplication_works_32bit_0() {
        let x = UnsignedInteger::from_u32(11_u32);
        let y = UnsignedInteger::from_u32(10_u32);
        let m = UnsignedInteger::from_u32(23_u32);
        let mu: u32 = 43691; // negative of the inverse of `m` modulo 2^32
        let c = UnsignedInteger::from_u32(13_u32); // x * y * (r^{-1}) % m, where r = 2^{32 * NUM_LIMBS}
        assert_eq!(MontgomeryAlgorithms::cios(&x, &y, &m, &mu), c);
    }

    #[test]
    fn montgomery_multiplication_works_32bit_1() {
        let x = UnsignedInteger::from_u32(0x05ed176d);
        let y = UnsignedInteger::from_u32(0x5f103b0b);
        let m = UnsignedInteger::from_u32(0xcdb06195); // this is prime
        let mu: u32 = 4027266669; // negative of the inverse of `m` modulo 2^32
        let c = UnsignedInteger::from_u32(0x8d65cdee); // x * y * (r^{-1}) % m, where r = 2^{32 * NUM_LIMBS}
        assert_eq!(MontgomeryAlgorithms::cios(&x, &y, &m, &mu), c);
    }

    // Add more tests for different inputs and scenarios.
}
