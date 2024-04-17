use num_bigint::BigUint;
use num_traits::identities::{Zero, One};


// In the Projective Short Weirstrass form: E: Y^2*Z = X^3 + AX*Z^2 + B*Z^3

#[derive(Clone, PartialEq)]
pub struct EllipticCurve {
    pub prime_field: BigUint,
    pub a: BigUint,
    pub b: BigUint
}

#[derive(Clone)]
pub struct EllipticCurveProjectivePoint {
    pub coordinates: [BigUint; 3],
    pub curve: EllipticCurve
}

impl EllipticCurveProjectivePoint {

    pub fn new(curve: &EllipticCurve, coordinates:&[BigUint;3]) -> Self {
        Self {curve:curve.clone(), coordinates:coordinates.clone()}
    }

    //point at infinity
    pub fn neutral_element(&self) -> Self{
        return Self::new(&self.curve, &[BigUint::zero(), BigUint::one(), BigUint::zero()])
    }

    // Algorithm 7 in moonmath manual
    pub fn add(&self, other: &Self) -> Self{

        if self.curve != other.curve{
            panic!("You must add points with similar curves");
        }

        if self.coordinates == [BigUint::zero(), BigUint::one(), BigUint::zero()]{
            return Self::new(&self.curve, &other.coordinates);
        } else if other.coordinates == [BigUint::zero(), BigUint::one(), BigUint::zero()] {
            return Self::new(&self.curve, &self.coordinates);
        } 
        let field = &self.curve.prime_field;
        let a = &self.curve.a;
        
        let x1 = &self.coordinates[0];
        let y1 = &self.coordinates[1];
        let z1 = &self.coordinates[2];

        let x2 = &other.coordinates[0];
        let y2 = &other.coordinates[1];
        let z2 = &other.coordinates[2];

        let u1 = (y2 * z1) % field;
        let u2 = (y1 * z2) % field;

        let v1 = (x2 * z1) % field;
        let v2 = x1 * z2 % field;

        let x3: BigUint;
        let y3: BigUint;
        let z3: BigUint;

        if v1 == v2 {
            if u1 != u2{
                return self.neutral_element();
            } else if y1 == &BigUint::zero() {
                return self.neutral_element();
            }
            let w = (
                a * z1.modpow(&BigUint::from(2 as u32), field) 
                + &BigUint::from(3 as u32)*x1.modpow(&BigUint::from(2 as u32), field)
            ) % field;

            let s = (y1 * z1) % field;
            let b = (x1*y1*&s) % field;
            let h = (
                w.modpow(&BigUint::from(2 as u32), field) 
                + ((field-(BigUint::from(8 as u32))*&b % field))
            ) % field;

            x3 = (BigUint::from(2 as u32) * &h * &s) % field;
            y3 = (
                w * ((BigUint::from(4 as u32) * &b) + (field - (&h% field))) 
                + (field - (
                    BigUint::from(8 as u32) 
                    * y1.modpow(&BigUint::from(2 as u32), field)
                    *s.modpow(&BigUint::from(2 as u32), field) % field
                    )
                )
            ) % field;

            z3 = (BigUint::from(8 as u32) * s.modpow(&BigUint::from(3 as u32), field)) % field;
        } else {
            let u = (&u1+(field - (&u2% field))) % field;
            let v = (&v1+(field - (&v2% field))) % field;
            let w = (z1*z2) % field;
            let a = (
                u.modpow(&BigUint::from(2 as u32), field) * &w
                + (field - (v.modpow(&BigUint::from(3 as u32), field) % field)) 
                + (field - ((BigUint::from(2 as u32) * v.modpow(&BigUint::from(2 as u32), field) * &v2) % field))
            ) % field;

            x3 = (&v * &a) % field;
            y3 = (
                &u * (v.modpow(&BigUint::from(2 as u32), field) * &v2 
                + (field - (&a% field))) 
                + (field - (v.modpow(&BigUint::from(3 as u32), field) * &u2% field))
            ) % field;

            z3 = (v.modpow(&BigUint::from(3 as u32), field) * &w) % field;

        }
        
        let coordinates = [x3,y3,z3] as [BigUint;3];
        return Self::new(&self.curve, &coordinates);

    }

    //double and add algorithm: an introduction to mathematical cryptography: Chapter 5.3.1
    pub fn multiply(&self, mut n: BigUint) -> Self {
        let mut q = self.clone();
        let mut r = self.neutral_element();

        while n > BigUint::zero() {
            if &n % BigUint::from(2 as u32) == BigUint::one() {
                r = q.add(&r);
            }
            q = q.add(&q.clone());
            n = &n / BigUint::from(2 as u32);
            println!("{}",n);
        }

        return r;
    }
}