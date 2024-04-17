use std::env;

mod point;

use point::{EllipticCurveProjectivePoint, EllipticCurve};
use num_bigint::BigUint;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Please specify a mode: add, or multiply.");
        return;
    }
    
    let mode = &args[1];

    let curve = EllipticCurve {
        prime_field: BigUint::from(17 as u32),
        a: BigUint::from(16 as u32),
        b: BigUint::from(1 as u32)
    };

    match mode.as_str() {
        "add" => {
            let x1 = BigUint::from(5 as u32);
            let y1 = BigUint::from(1 as u32);
            let x2 = BigUint::from(6 as u32);
            let y2 = BigUint::from(3 as u32);
            let coordinates_x_1 = [x1, y1, BigUint::from(1 as u32)];
            let coordinates_x_2 = [x2, y2, BigUint::from(1 as u32)];

            let point = EllipticCurveProjectivePoint::new(&curve, &coordinates_x_1);
            let second_point = EllipticCurveProjectivePoint::new(&curve, &coordinates_x_2);

            let point_at_infinity = point.neutral_element();
            let coordinates = point_at_infinity.coordinates;
            println!("point at infinity:{},{},{}", coordinates[0], coordinates[1], coordinates[2]);
            println!("doubling");
            let new_point = point.add(&point.clone());
            let coordinates = new_point.coordinates;
            println!("x3:{},y3:{},z3:{}", coordinates[0], coordinates[1], coordinates[2]);
            println!("adding");
            let new_point = point.add(&second_point);
            let coordinates = new_point.coordinates;
            println!("x3:{},y3:{},z3:{}", coordinates[0], coordinates[1], coordinates[2]);
            println!("adding point at infinity");
            let new_point = point.add(&point.neutral_element());
            let coordinates = new_point.coordinates;
            println!("x3:{},y3:{},z3:{}", coordinates[0], coordinates[1], coordinates[2]);
            println!("adding inverse");
            let x2 = BigUint::from(5 as u32);
            let y2 = BigUint::from(16 as u32);
            let coordinates_x_2 = [x2, y2, BigUint::from(1 as u32)];
            let second_point = EllipticCurveProjectivePoint::new(&curve, &coordinates_x_2);
            let new_point = point.add(&second_point);
            let coordinates = new_point.coordinates;
            println!("x3:{},y3:{},z3:{}", coordinates[0], coordinates[1], coordinates[2]);
            
        }
        "multiply" => {
            let x1 = BigUint::from(5 as u32);
            let y1 = BigUint::from(1 as u32);
            let coordinates_x_1 = [x1, y1, BigUint::from(1 as u32)];
            let point = EllipticCurveProjectivePoint::new(&curve, &coordinates_x_1);

            let new_point = point.multiply(BigUint::from(70 as u32));

            let coordinates = new_point.coordinates;
            println!("x3:{},y3:{},z3:{}", coordinates[0], coordinates[1], coordinates[2]);
        }
        _ => {
            println!("Invalid mode. Use 'add', or 'multiply'.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_neutral_element() {
        let curve = EllipticCurve {
            prime_field: 17.to_biguint().unwrap(),
            a: 16.to_biguint().unwrap(),
            b: 1.to_biguint().unwrap(),
        };
        let point = EllipticCurveProjectivePoint::new(&curve, &[0.to_biguint().unwrap(), 1.to_biguint().unwrap(), 0.to_biguint().unwrap()]);
        let neutral_point = point.neutral_element();
        assert_eq!(neutral_point.coordinates, [0.to_biguint().unwrap(), 1.to_biguint().unwrap(), 0.to_biguint().unwrap()]);
    }

    #[test]
    fn test_add() {
        let curve = EllipticCurve {
            prime_field: 17.to_biguint().unwrap(),
            a: 16.to_biguint().unwrap(),
            b: 1.to_biguint().unwrap(),
        };
        let point1 = EllipticCurveProjectivePoint::new(&curve, &[5.to_biguint().unwrap(), 1.to_biguint().unwrap(), 1.to_biguint().unwrap()]);
        let point2 = EllipticCurveProjectivePoint::new(&curve, &[6.to_biguint().unwrap(), 3.to_biguint().unwrap(), 1.to_biguint().unwrap()]);
        let expected_result = EllipticCurveProjectivePoint::new(&curve, &[9.to_biguint().unwrap(), 16.to_biguint().unwrap(), 1.to_biguint().unwrap()]);
        let result = point1.add(&point2);
        assert_eq!(result.coordinates, expected_result.coordinates);
    }

    #[test]
    fn test_multiply() {
        let curve = EllipticCurve {
            prime_field: 17.to_biguint().unwrap(),
            a: 16.to_biguint().unwrap(),
            b: 1.to_biguint().unwrap(),
        };
        let point = EllipticCurveProjectivePoint::new(&curve, &[5.to_biguint().unwrap(), 1.to_biguint().unwrap(), 1.to_biguint().unwrap()]);
        let expected_result = EllipticCurveProjectivePoint::new(&curve, &[9.to_biguint().unwrap(), 16.to_biguint().unwrap(), 1.to_biguint().unwrap()]);
        let result = point.multiply(3.to_biguint().unwrap());
        assert_eq!(result.coordinates, expected_result.coordinates);
    }
}

#[test]
fn test_main_invalid_mode() {
    let args: Vec<String> = Vec::new();
    let result = std::panic::catch_unwind(|| {
        main();
    });
    assert!(result.is_err());
}