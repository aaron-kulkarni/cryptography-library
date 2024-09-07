use num_bigint::BigUint;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn decrypt() -> Result<(), Box<dyn Error>> {
    let msg_file = File::open("rsamsg.txt")?;
    let msg_reader = BufReader::new(msg_file);
    let cipher = msg_reader
        .lines()
        .next()
        .ok_or("Error with cipher")??
        .parse::<BigUint>()?;

    let key_file = File::open("rsaprivate.txt")?;
    let reader = BufReader::new(key_file);

    let mut lines = reader.lines();

    let n = lines
        .next()
        .ok_or("Error with first line")??
        .parse::<BigUint>()?;
    let d = lines
        .next()
        .ok_or("Missing second number")??
        .parse::<BigUint>()?;

    let result = cipher.modpow(&d, &n).to_bytes_be();

    let mut result_file = File::create("decryptedstuff.txt")?;
    result_file.write_all(&result)?;

    return Ok(());
}

fn decrypt_block(vec: &mut Vec<u8>, n: &BigUint, d: usize, start: usize) -> Vec<u8> {
    let d_big = BigUint::from(d);
    let v_big = BigUint::from_bytes_be(&vec[start..start + 128]);
    return v_big.modpow(&d_big, n).to_bytes_be();
}
