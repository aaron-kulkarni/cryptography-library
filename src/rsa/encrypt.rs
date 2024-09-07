use num_bigint::BigUint;
use num_primes::Generator;
use num_traits::CheckedMul;
use num_traits::ToPrimitive;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::{BufRead, BufReader};

pub fn encrypt(vec: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut padding = 128 - (vec.len() % 128);
    if padding == 128 {
        padding = 0;
    }

    for _ in 0..padding {
        vec.push(padding as u8);
    }

    let msg_file = File::open("rsapublic.txt")?;
    let reader = BufReader::new(msg_file);

    let mut lines = reader.lines();
    let n = lines
        .next()
        .ok_or("Error with first line")??
        .parse::<BigUint>()?;

    let e = lines
        .next()
        .ok_or("Error with second line")??
        .parse::<BigUint>()?;

    let text = BigUint::from_bytes_be(&vec);
    let result = text.modpow(&e, &n);

    let result_file = File::create("rsamsg.txt")?;
    let mut result_writer = BufWriter::new(result_file);
    writeln!(result_writer, "{}", result)?;

    return Ok(());
}
