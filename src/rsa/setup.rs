use num_primes::BigUint;
use num_primes::Generator;
use num_traits::ToPrimitive;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::{fs::File, io::Error};

// p = large prime
// q = large prime
// n = p x q
// r = (p-1)(q-1)
// e = 3, 5, 17, 65537
// d = e-1 * mod(r)

pub fn rsa_setup() -> Result<(), Error> {
    let (p, q) = make_p_q();
    let n = p.clone() * q.clone();
    let e: BigUint = BigUint::from(65537 as u32);
    let r: BigUint = (p - BigUint::from(1 as u32)) * (q - BigUint::from(1 as u32));
    let d: BigUint = (e.clone() - BigUint::from(1 as u32)) % r;

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

    return (p, q);
}
