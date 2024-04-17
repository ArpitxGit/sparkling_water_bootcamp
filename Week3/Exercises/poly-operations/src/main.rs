extern crate num;
extern crate num_bigint;
extern crate ndarray;
extern crate nalgebra;
extern crate rug;
extern crate rayon;

use num::{BigInt, Integer};
use ndarray::Array1;
use nalgebra::{Vector2, Matrix2};
use rug::Integer as RugInteger;
use rayon::prelude::*;

// Define a polynomial structure using BigInt from the num-bigint crate
struct Polynomial {
    coeffs: Vec<BigInt>,
}

impl Polynomial {
    // Constructor for creating a polynomial with BigInt coefficients
    fn new(coeffs: Vec<BigInt>) -> Polynomial {
        Polynomial { coeffs }
    }

    // Evaluate the polynomial at a given integer value using BigInt arithmetic
    fn eval(&self, x: &BigInt) -> BigInt {
        let mut result = BigInt::zero();
        let mut x_power = BigInt::one();

        for coeff in self.coeffs.iter() {
            result += coeff * &x_power;
            x_power *= x;
        }

        result
    }
}

fn main() {
    // Create polynomials with BigInt coefficients
    let p1 = Polynomial::new(vec![BigInt::from(3), BigInt::from(2), BigInt::from(1)]); // 1x^2 + 2x + 3
    let p2 = Polynomial::new(vec![BigInt::from(5), BigInt::zero(), BigInt::from(2)]); // 2x^2 + 5

    // Evaluate p1 at x = 2 using BigInt arithmetic
    let x = BigInt::from(2);
    let evaluation = p1.eval(&x);

    println!("Evaluation of p1 at x = {}: {}", x, evaluation);

    // Example usage of other crates
    let mut v = Vector2::new(3.0, 4.0); // Vector from nalgebra crate
    v += Vector2::new(1.0, 2.0);

    let a = Array1::from(vec![1, 2, 3]); // Array from ndarray crate
    let sum = a.sum();

    let m = Matrix2::new(1.0, 2.0, 3.0, 4.0); // Matrix from nalgebra crate
    let determinant = m.determinant();

    let mut int = RugInteger::from(42); // Integer from rug crate
    int += 10;

    let result: Vec<_> = (0..10).into_par_iter().map(|x| x * x).collect(); // Parallel computation with rayon

    println!("Vector v: {:?}", v);
    println!("Sum of array a: {}", sum);
    println!("Determinant of matrix m: {}", determinant);
    println!("Updated integer int: {}", int);
    println!("Parallel computation result: {:?}", result);
}