use num::BigUint;
use num_bigint;
use num_bigint::BigInt;
use num_bigint::Sign;
use num_primes::Generator;
use num_traits::ToPrimitive;
use num_traits::{One, Zero};

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
fn mult_inverse(a: &BigUint, b: &BigUint) -> BigUint {
    // mult inverse of b under a using Extended Euclidean Algorithm
    let mut s0: BigInt = Zero::zero();
    let mut s1: BigInt = One::one();
    let mut r0 = a.clone();
    let mut r1 = b.clone();

    while r1 != Zero::zero() {
        let r2 = &r0 - (&r0 / &r1) * &r1;
        let s2 = &s0 - BigInt::from_biguint(Sign::Plus, &r0 / &r1) * &s1;
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
    }

    while s0 < Zero::zero() {
        s0 = s0 + BigInt::from_biguint(Sign::Plus, a.clone());
    }
    s0.to_biguint()
        .expect("Error converting to unsigned integer")
}

pub fn rsa_setup() -> Result<(), Error> {
    println!("Generating new RSA keys can take a while. Please be patient.");
    let (p, q) = make_p_q();
    // let n = &p * &q;
    // let phi = (p - BigUint::one()) * (q - BigUint::one());
    // let e = BigUint::from(65537u32); // Common choice for e
    // let d = mod_inverse(&e, &phi);
    let n = &p * &q;
    let e: BigUint = BigUint::from(65537u32);
    let r: BigUint = (p - BigUint::from(1u32)) * (q - BigUint::from(1u32));
    let d = mult_inverse(&e, &r);

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
