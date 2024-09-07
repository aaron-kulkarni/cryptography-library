use num::BigUint;
use num_bigint;
use num_bigint::BigInt;
use num_primes::Generator;
use num_traits::{One, Zero};
use std::io::{BufWriter, Write};
use std::{fs::File, io::Error};

fn mult_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a, m);
    if g == BigInt::one() {
        Some((x % m + m) % m)
    } else {
        None
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
    let n = &p * &q;
    let e: BigUint = BigUint::from(65537u32);
    let r: BigUint = (p - BigUint::one()) * (q - BigUint::one());
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
