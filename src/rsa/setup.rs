use num::BigUint;
use num_bigint;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_primes::Generator;
use num_traits::ToPrimitive;
use num_traits::{One, Zero};
use std::ops::Sub;

use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::{fs::File, io::Error};

// p = large prime
// q = large prime
// n = p x q
// r = (p-1)(q-1)
// e = 3, 5, 17, 65537
// d = e-1 * mod(r)
//

// fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
//     if a.is_zero() {
//         (b.clone(), BigUint::zero(), BigUint::from(1u32))
//     } else {
//         let (g, x, y) = extended_gcd(&(b % a), a);
//         let times = b / a;
//         if y > times.clone() * x.clone() {
//             (g, y - times * x.clone(), x)
//         } else {
//             (g, x - times * y.clone(), y)
//         }
//     }
// }

// fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
//     let (g, x, _) = extended_gcd(a, m);
//     if g == BigUint::from(1u32) {
//         Some((x % m + m) % m) // Ensure it is positive
//     } else {
//         None // Inverse doesn't exist
//     }
// }

// {
//     uint32_t eprev, dprev, d = 1, etemp, dtemp;

//     eprev = phi, dprev = phi;
//     while (e != 1)
//     {
// 	    etemp = e;
// 	    dtemp = d;
// 	    e = eprev - eprev / etemp * e;
// 	    d = dprev - eprev / etemp * d;
// 	    eprev = etemp;
// 	    dprev = dtemp;
// 	    while (d < 0)
// 		    d += phi;
//     }
//     return d;
// }
//
// fn mult_inverse(a: &BigUint, b: &BigUint) -> BigUint {
//     // mult inverse of b under a using Extended Euclidean Algorithm
//     let mut s0: BigInt = Zero::zero();
//     let mut s1: BigInt = One::one();
//     let mut r0 = a.clone();
//     let mut r1 = b.clone();

//     while r1 != Zero::zero() {
//         let r2 = &r0 - (&r0 / &r1) * &r1;
//         let s2 = &s0 - BigInt::from_biguint(Sign::Plus, &r0 / &r1) * &s1;
//         r0 = r1;
//         r1 = r2;
//         s0 = s1;
//         s1 = s2;
//     }

//     while s0 < Zero::zero() {
//         s0 = s0 + BigInt::from_biguint(Sign::Plus, a.clone());
//     }
//     s0.to_biguint()
//         .expect("Error converting to unsigned integer")
// }
//

// fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
//     if *a == BigUint::zero() {
//         (b.clone(), BigUint::zero(), BigUint::one())
//     } else {
//         let (g, x, y) = extended_gcd(&(b % a), a);
//         (g, y.clone() - ((b / a.clone()) * &x), x)
//     }
// }

// fn mult_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
//     let (g, x, _) = extended_gcd(a, m);
//     if g == BigUint::one() {
//         Some((x % m + m) % m)
//     } else {
//         None
//     }
// }

// fn mult_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
//     let (gcd, x, _) = extended_gcd(a, m);
//     if gcd.is_one() {
//         // Make sure the result is positive
//         let result = (x % m + m) % m;
//         Some(result)
//     } else {
//         None
//     }
// }

// /// Extended Euclidean Algorithm to find gcd and coefficients
// /// such that gcd(a, b) = a * x + b * y
// fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
//     let mut old_r = a.clone();
//     let mut r = b.clone();
//     let mut old_s = BigUint::one();
//     let mut s = BigUint::zero();
//     let mut old_t = BigUint::zero();
//     let mut t = BigUint::one();

//     while !r.is_zero() {
//         let quotient = &old_r / &r;
//         let remainder = &old_r % &r;
//         old_r = r.clone();
//         r = remainder;

//         let s_temp = s.clone();
//         s = old_s.sub(&quotient * &s);
//         old_s = s_temp;

//         let t_temp = t.clone();
//         t = old_t.sub(&quotient * &t);
//         old_t = t_temp;
//     }

//     (old_r, old_s, old_t)
// }

fn mult_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g == BigInt::one() {
        Some((x % m + m) % m) // Ensure it is positive
    } else {
        None // Inverse doesn't exist
    }
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if *a == BigInt::zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = extended_gcd(&(b % a), a);
        (g, y.clone() - ((b / a.clone()) * &x), x)
    }
}
pub fn rsa_setup() -> Result<(), Error> {
    println!("Generating new RSA keys can take a while. Please be patient.");
    let (p, q) = make_p_q();
    println!("{\n\n}", p);
    println!("{\n\n}", q);
    // let n = &p * &q;
    // let phi = (p - BigUint::one()) * (q - BigUint::one());
    // let e = BigUint::from(65537u32); // Common choice for e
    // let d = mod_inverse(&e, &phi);
    let n = &p * &q;
    println!("{\n\n}", n);
    let e: BigUint = BigUint::from(65537u32);
    let r: BigUint = (p - BigUint::from(1u32)) * (q - BigUint::from(1u32));
    println!("{}", r);
    // let d = mult_inverse(&e, &r);
    // println!("{}", d);
    let d = match mult_inverse(&BigInt::from(e.clone()), &BigInt::from(r)) {
        Some(x) => x,
        None => panic!("Couldn't setup rsa keys. Errored during mult inverse."),
    };

    let pub_file = match File::create("rsapublic.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Could not create rsapublic.txt.");
            return Err(e);
        }
    };

    let mut pub_writer = BufWriter::new(pub_file);
    writeln!(pub_writer, "{}", n)?;
    writeln!(pub_writer, "{}", e)?;

    let priv_file = match File::create("rsaprivate.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Could not create rsaprivate.txt.");
            return Err(e);
        }
    };

    let mut priv_writer = BufWriter::new(priv_file);
    writeln!(priv_writer, "{}", n)?;
    writeln!(priv_writer, "{}", d)?;

    return Ok(());
}

fn make_p_q() -> (BigUint, BigUint) {
    let p = Generator::new_prime(1024);
    let q = Generator::new_prime(1024);

    return (
        BigUint::from_bytes_be(&p.to_bytes_be()),
        BigUint::from_bytes_be(&q.to_bytes_be()),
    );
}
