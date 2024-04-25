use rand::{thread_rng, Rng};
use std::collections::HashSet;

// Define a struct for a point on the polynomial
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

// Generate coefficients for the polynomial
fn generate_coefficients(secret: i64, threshold: usize, total_shares: usize) -> Vec<i64> {
    println!("Generating coefficients...");
    let mut coefficients = vec![secret];
    let mut rng = thread_rng();

    for _ in 1..threshold {
        coefficients.push(rng.gen_range(1..100)); // Generate random coefficients
    }

    while coefficients.len() < total_shares {
        let next_coeff = rng.gen_range(1..100);
        let mut new_coef = next_coeff;
        for coef in &coefficients {
            new_coef ^= coef;
        }
        coefficients.push(new_coef);
    }

    coefficients
}

// Evaluate the polynomial at a given x coordinate
fn evaluate_polynomial(coefficients: &[i64], x: i64) -> i64 {
    println!("Evaluating polynomial...");
    let mut result = 0;

    for &coef in coefficients.iter().rev() {
        result = result * x + coef;
    }

    result
}

// Generate shares based on the polynomial
fn generate_shares(secret: i64, threshold: usize, total_shares: usize) -> Vec<Point> {
    let coefficients = generate_coefficients(secret, threshold, total_shares);
    let mut shares = Vec::new();
    let mut _rng = thread_rng();

    for i in 1..=total_shares {
        let x = i as i64;
        let y = evaluate_polynomial(&coefficients, x);
        shares.push(Point { x, y });
    }

    shares
}

fn recover_secret(shares: &[Point], threshold: usize) -> Option<i64> {
    if shares.len() < threshold {
        return None; // Not enough shares to recover the secret
    }

    let mut secret = 0;

    for i in 0..shares.len() {
        let mut numerator = 1;
        let mut denominator = 1;

        for j in 0..shares.len() {
            if i != j {
                numerator *= -shares[j].x;
                denominator *= shares[i].x - shares[j].x;
            }
        }

        let coef = numerator / denominator;
        secret += shares[i].y * coef;
    }

    Some(secret)
}



fn main() {
    let secret = 42;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares(secret, threshold, total_shares);
    println!("Shares: {:?}", shares);

    let recovered_secret = recover_secret(&shares, threshold);
    match recovered_secret {
        Some(secret) => println!("Recovered Secret: {}", secret),
        None => println!("Failed to recover secret!"),
    }
}

#[test]
fn test_secret_sharing() {
    let secret = 42;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares(secret, threshold, total_shares);
    println!("Shares: {:?}", shares);

    let recovered_secret = recover_secret(&shares[..threshold], threshold);
    println!("Recovered Secret: {:?}", recovered_secret);

    assert_eq!(recovered_secret, Some(secret));
}
